#include <ESP8266WiFi.h>
#include <WiFiClient.h>
#include <ESP8266WebServer.h>
#include <ArduinoJson.h>
#include <SPI.h>
#include <sdCrud.h>

const char *ssid = "one-legged pirate";
const char *password = "qwert222";

ESP8266WebServer server(80);

File32 activeFile;

String get_file_path()
{
  return server.arg("path");
}

void get_directory()
{
  File32 root = SD.open("/");
  String res = printDirectory(root, 0);
  root.close();
  server.send(200, "text/plain", res);
}

void get_file()
{
  String path  = get_file_path();

  File32 f = SD.open(path, FILE_READ);
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

  if (!SD.begin(D8))
  {
    Serial.println("initialization failed!");
    return;
  }
  Serial.println("initialization done.");

  Serial.println("done!");

  WiFi.begin(ssid, password);
  IPAddress apip = WiFi.localIP();
  Serial.print("visit: \n");
  Serial.println(apip);

  server.on("/path", get_directory);
  server.on("/file", HTTP_GET, get_file);
  server.on("/mem", HTTP_GET, []()
  {
    String s = String(get_total_space());
    server.send(200, "text/plain", s);
  } );
  server.on("/memfree", HTTP_GET, []()
  {
    String s = String(get_free_space());
    server.send(200, "text/plain", s);
  });
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
