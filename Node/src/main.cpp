#include <ESP8266WiFi.h>
#include <WiFiClient.h>
#include <ESP8266WebServer.h>
#include <SPI.h>
#include <sdCrud.h>
#include <ArduinoJson.h>

ESP8266WebServer server(80);

File activeFile;

const String CONFIG_PATH = "/.storage/config";

String get_file_path()
{
  return server.arg("path");
}

void get_directory()
{
  File root = SD.open("/");
  String res = printDirectory(root, 0);
  root.close();
  server.send(200, "text/plain", res);
}

void get_file()
{
  String path  = get_file_path();

  File f = SD.open(path, FILE_READ);
  if (!f.available())
  {
    server.send(404, "text/plain", "file not found");
    return;
  }

  String dataType = "text/plain";

    if (path.endsWith(".src")) {
    path = path.substring(0, path.lastIndexOf("."));
  } else if (path.endsWith(".htm")) {
    dataType = "text/html";
  } else if (path.endsWith(".css")) {
    dataType = "text/css";
  } else if (path.endsWith(".js")) {
    dataType = "application/javascript";
  } else if (path.endsWith(".png")) {
    dataType = "image/png";
  } else if (path.endsWith(".gif")) {
    dataType = "image/gif";
  } else if (path.endsWith(".jpg")) {
    dataType = "image/jpeg";
  } else if (path.endsWith(".ico")) {
    dataType = "image/x-icon";
  } else if (path.endsWith(".xml")) {
    dataType = "text/xml";
  } else if (path.endsWith(".pdf")) {
    dataType = "application/pdf";
  } else if (path.endsWith(".zip")) {
    dataType = "application/zip";
  }
  Serial.print(server.streamFile(f, dataType));
  f.close();
}

void upload_file()
{
  HTTPUpload &upload = server.upload();
  if (upload.status == UPLOAD_FILE_START)
  {
    String path = get_file_path();
    if(SD.exists(path))
      SD.remove(path);
    activeFile = SD.open(path, FILE_WRITE);
  }
    

  if (upload.status == UPLOAD_FILE_WRITE)
  {
    activeFile.write(upload.buf, upload.currentSize);
  }
  if (upload.status == UPLOAD_FILE_END)
    activeFile.close();
}

void returnOK() {
  server.send(200, "text/plain", "");
}

void sendCode(int code)
{
  server.send(code, "text/plain", String("exit with code: ") + String(code));
}

void setup()
{
  Serial.begin(9600);
  delay(1000);

  Serial.println("Initializing SD card...");
  if (!SD.begin(D8, 500000))
  {
    Serial.println("initialization failed!");
    return;
  }
  Serial.println("initialization done.");

  Serial.println("done!");

  String ssid = String("device-") + WiFi.macAddress();
  String pas = "12345678";

  if(!SD.exists(CONFIG_PATH))
  {
    Serial.printf("config doesnt exists\n %s\n%s ", ssid.c_str(), pas.c_str());
    WiFi.softAP(ssid, pas);
    WiFi.softAPConfig(IPAddress(192,168,20,1), IPAddress(192,168,20,1), IPAddress(255,255,255,0));
  }
  else
  {
    File f = SD.open(CONFIG_PATH);
    ssid = f.readStringUntil('\n');
    pas = f.readStringUntil('\n');
    String ip = f.readStringUntil('\n');
    String gateway = f.readStringUntil('\n');
    IPAddress mask = IPAddress(255,255,255,0);
    IPAddress ip_a;
    IPAddress gateway_a;
    ip_a.fromString(ip);
    gateway_a.fromString(gateway);

    WiFi.config(ip_a, gateway_a, mask);
    wl_status_t status =  WiFi.begin(ssid, "qwert222");
    switch(status)
    {
    case WL_NO_SSID_AVAIL:
      Serial.print("Wifi not found");

      break;
    case WL_WRONG_PASSWORD:
      Serial.printf("Wrong password");
    case WL_CONNECT_FAILED:
      Serial.printf("Connection failed");
      break;
    }

    if(status != WL_NO_SSID_AVAIL && status != WL_WRONG_PASSWORD && status != WL_CONNECT_FAILED)
      Serial.printf("config loaded\n %s\n%s\n", ssid.c_str(), pas.c_str());
    else
    {
      SD.remove(CONFIG_PATH);
      ESP.restart();
    }
  }
  

  server.on("/path", get_directory);

  server.on("/config",HTTP_POST, []()
  {
     if (server.hasArg("plain")== false){ //Check if body received
 
            server.send(400, "text/plain", "Body not received");
            return;
      }
      String message = server.arg("plain");
      StaticJsonDocument<512> doc;
      deserializeJson(doc, message);
      Serial.printf(message.c_str());
      String ssid = doc["ssid"];
      String pass = doc["pass"];
      String ip = doc["ip"];
      String gateway = doc["gateway"];
      Serial.printf("%s %s %s\n", ssid.c_str(), pass.c_str(), ip.c_str());

      if(SD.exists(CONFIG_PATH))
        SD.remove(CONFIG_PATH);

      File f = SD.open(CONFIG_PATH, FILE_WRITE);
      f.write((ssid + "\n").c_str());
      f.write((pass + "\n").c_str());
      f.write((ip + "\n").c_str());
      f.write((gateway + "\n").c_str());
      f.close();
      returnOK();
      delay(1000);
      ESP.restart();
  });

  server.on("/info", HTTP_GET, []()
  {
    DynamicJsonDocument doc(256);
    doc["ip"] = WiFi.localIP().toString();
    doc["mac"] = WiFi.macAddress();
    doc["type"] = "node";
    String s;
    serializeJson(doc, s);

    server.send(200, "application/json", s);
  });
  server.on("/file", HTTP_GET, get_file);
  server.on("/mem", HTTP_GET, []()
  {
    uint64_t total = get_total_space();
    uint64_t free = get_free_space();
    DynamicJsonDocument doc(256);
    doc["total"] = total;
    doc["free"] = free;
    String s;
    serializeJson(doc, s);
    server.send(200, "application/json", s);
  } );

  server.on("/file",HTTP_POST,[]() {returnOK();}, upload_file);
  server.on("/file", HTTP_PUT,[]() 
  {
    bool res = move_file(get_file_path(), server.arg("new path"));
    if(res)
      returnOK();
    sendCode(400);
  });
  server.on("/file", HTTP_DELETE,[]() 
  {
    if(delete_file(get_file_path()))
      returnOK();
    sendCode(400);
  });
  server.begin();
  Serial.println("HTTP server beginned");
}

void loop()
{
  server.handleClient();
}
