#include <ESP8266WiFi.h>
#include <WiFiClient.h>
#include <ESP8266WebServer.h>
#include <ArduinoJson.h>
#include <SD.h>
#include <SPI.h>

const char *ssid = "one-legged pirate";
const char *password = "qwert222";

ESP8266WebServer server(80);
WiFiClient client;

const char *weatherHost = "api.weatherapi.com";

const String HtmlHtml = "<html>"
                        "<head>"
                        "<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\" /></head>"
                        "<body>"
                        "<h1>hello from esp8266</h1>"
                        "<button class='btn' onclick = 'click'>click me</button>"
                        "</body>"
                        "<style> .btn{color:red;} </style>";

void response()
{
  String htmlRes = HtmlHtml;

  server.send(200, "text/html", htmlRes);
}

void weather()
{
  if (client.connect(weatherHost, 80))
  {
    client.print(String("GET /v1/current.json?key=e757f625ce974ca0b0f212252230609&q=Moscow") + " HTTP/1.1\r\n" +
                 "Host: " + weatherHost + "\r\n" +
                 "\r\n");
    String res = client.readString();
    String json = res.substring(res.indexOf("{"), res.lastIndexOf("}") + 1);
   //Serial.println(json);
    StaticJsonDocument<1024> jsonDoc;
    DeserializationError err = deserializeJson(jsonDoc, json);
    //Serial.println(err.c_str());
    client.stop();
    float temp = jsonDoc["current"]["temp_c"];
    //Serial.println(temp);
    server.send(200, "text/html", "<H1>Temperature now: " + String(temp) + "</H1>");
  }
  else
  {
    server.send(200, "text/plain", "error");
  }
}

String printDirectory(File dir, int numTabs) {
  String res = "";
  while (true) {

    File entry =  dir.openNextFile();
    if (! entry) {
      // no more files
      break;
    }
    for (uint8_t i = 0; i < numTabs; i++) {
      res+="\t";
    }
    if (entry.isDirectory()) {
      res += "/\n";
      res += printDirectory(entry, numTabs + 1);
    } else {
      // files have sizes, directories do not
      res+="\t\t";
      res+= String(entry.name()) + "\n";
    }
    entry.close();
  }
  return res;
}

void serverPath()
{
  File root = SD.open("/");
  String res = printDirectory(root, 0);
  root.close();
  server.send(200, "text/plain", res);
}

void getFile()
{
  File f = SD.open("test.txt", FILE_READ);
  server.sendContent(f, f.size());
  f.close();
}

void writeFile()
{
  File myfile = SD.open("test.txt", FILE_WRITE);
  myfile.write("hello from esp8266");
  myfile.close();
}

void setup()
{
  Serial.begin(9600);
  delay(1000);

  Serial.println("Initializing SD card...");

  if (!SD.begin(D8)) {
    Serial.println("initialization failed!");
    return;
  }
  Serial.println("initialization done.");

  //writeFile();

  

  Serial.println("done!");

  WiFi.begin(ssid, password);
  IPAddress apip = WiFi.localIP();
  Serial.print("visit: \n");
  Serial.println(apip);
  server.on("/", response);
  server.on("/weather", weather);
  server.on("/path", serverPath);
  server.on("/file", getFile);
  server.begin();
  Serial.println("HTTP server beginned");
}

void loop()
{
  server.handleClient();
}

