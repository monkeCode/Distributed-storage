#include <SdFat.h>

SdFat SD;

String get_file_name(File32 file)
{

  char* buf = new char[20];
  file.getName(buf,20);
  String name = String(buf);
  delete[] buf;
  return name;
}

long get_total_space()
{
  return SD.clusterCount() * SD.vol()->sectorsPerCluster()/2;
}

long get_free_space()
{
  return SD.vol() -> freeClusterCount() * SD.vol()->sectorsPerCluster()/2;
  }

bool create_dir(String path)
{
  return SD.mkdir(path);
}

String printDirectory(File32 dir, int numTabs)
{
  String res = "";
  while (true)
  {

    File32 entry = dir.openNextFile();
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
      res += "/"+ get_file_name(entry) +"\n";
      res += printDirectory(entry, numTabs + 1);
    }
    else
    {
      res += "- ";
      res += get_file_name(entry) + "\n";
    }
    entry.close();
  }
  return res;
}

bool move_file(String oldPath, String newPath)
{
  File32 f = SD.open(oldPath, FILE_READ);
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

  File32 f = SD.open(path, FILE_READ);
  if(!f.isDirectory())
  {
    f.close();
    return SD.remove(path);
  }

  f.rewindDirectory();
  while (true) 
  {
    File32 entry = f.openNextFile();
    
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
