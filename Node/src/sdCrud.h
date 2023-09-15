#include <SD.h>

int get_total_space()
{
  return SD.size();
}

int get_free_space()
{
  return get_total_space() - SD.clusterSize()* SD.totalClusters();
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
      res += "/"+ String(entry.name()) +"\n";
      res += printDirectory(entry, numTabs + 1);
    }
    else
    {
      res += "- ";
      res += String(entry.name()) + "\n";
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
    if(!SD.exists(dirPath))
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
    String entryPath = path + "/" + entry.name();
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
