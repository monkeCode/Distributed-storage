#include <SD.h>
#include <ArduinoJson.h>

namespace sdApi
{
  uint64_t get_size(File file)
  {
    if(file.isDirectory())
    {
      uint64_t size = 0;
      while(true)
      {
        File f = file.openNextFile();
        if(!f)
          return size;
        size += get_size(f);
        f.close();
      }
    }
      return file.size();
  }

  String get_file_name(File file)
  {
    return String(file.name());
  }

  uint64_t get_total_space()
  {
    return SD.size64();
  }

  uint64_t get_free_space()
  {
    File f = SD.open("/");
    uint64_t size = get_size(f);
    f.close();
    return get_total_space() - size;
  }

  bool create_dir(String path)
  {
    return SD.mkdir(path);
  }

  void printDirectory(JsonArray& doc, File& dir, unsigned int numTabs, unsigned int max_depth)
  {
    while (true)
    {
      File entry = dir.openNextFile();
      if (!entry)
      {
        break;
      }

      DynamicJsonDocument obj(entry.isDirectory()?2048:256);
      obj["isDirectory"] = entry.isDirectory();
      obj["name"] = get_file_name(entry);
      obj["lastModificated"] = entry.getLastWrite();
      obj["creatonDate"] = entry.getCreationTime();
      obj["size"] = entry.size();

      if (entry.isDirectory() && numTabs < max_depth)
      {
          DynamicJsonDocument files(2048);
          JsonArray arr = files.to<JsonArray>();
          printDirectory(arr, entry, numTabs + 1, max_depth);
          files.shrinkToFit();
          obj["files"] = files;
      }
      obj.shrinkToFit();
      doc.add(obj);
      entry.close();
    }
  }

  bool move_file(String oldPath, String newPath)
  {
    File f = SD.open(oldPath, FILE_READ);
    if(newPath[0] != '/')
    {
      newPath = "/" + newPath;
    }

    if(f.available())
    {
      String name = f.name();
      f.close();
      String dirPath = newPath.substring(0, newPath.lastIndexOf("/"));

      if(newPath[newPath.length()-1] == '/')
      {
        newPath += name;
      }
      if(dirPath.length() > 0)
      {
        if(!SD.exists(dirPath))
          create_dir(dirPath);
        return SD.rename(oldPath, newPath);
      }
        return SD.rename(oldPath, newPath);
    }
    return false;
    
  }

  bool delete_file(String path)
  {
    if(!SD.exists(path))
      return true;

    File f = SD.open(path, FILE_READ);
    if(!f.isDirectory())
    {
      f.close();
      return SD.remove(path);
    }

    f.rewindDirectory();
    while (true) 
    {
      File entry = f.openNextFile();
      
      if (!entry) 
      {
        break;
      }
      String entryPath = path + "/" + get_file_name(entry);
      if (entry.isDirectory()) 
      {
        entry.close();
        if(!delete_file(entryPath))
          return false;
      } 
      else 
      {
        entry.close();
        SD.remove(entryPath);
      }
      yield();
    }
    f.close();
    return SD.rmdir(path);
    }

  bool begin(int pin=D8)
  {
    return SD.begin(pin, 500000);
  }
}

