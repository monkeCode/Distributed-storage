#include <SD.h>
#include <SPI.h>

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

String printDirectory(File dir, int numTabs)
{
  String res = "";
  while (true)
  {

    File entry = dir.openNextFile();
    if (!entry)
    {
      // no more files
      break;
    }
    for (uint8_t i = 0; i < numTabs; i++)
    {
      res += "\t";
    }
    if (entry.isDirectory())
    {
      res += "/"+ get_file_name(entry) + "\n";
      res += printDirectory(entry, numTabs + 1);
    }
    else
    {
      res += "- ";
      size_t s = entry.size();
      res += get_file_name(entry) +" "+ s +" bytes" + "\n";
    }
    entry.close();
  }
  return res;
}

bool move_file(String oldPath, String newPath)
{
  File f = SD.open(oldPath, FILE_READ);
  if(f.available())
  {
    f.close();
    String dirPath = newPath.substring(0, newPath.lastIndexOf("/"));
    if(newPath.lastIndexOf("/") >0 && !SD.exists(dirPath))
      create_dir(dirPath);
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
