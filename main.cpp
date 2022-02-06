// `exec` borrowed from
// https://www.tutorialspoint.com/How-to-execute-a-command-and-get-the-output-of-command-within-Cplusplus-using-POSIX

#include <iostream>
#include <stdexcept>
#include <stdio.h>
#include <string>

using namespace std;

string exec(string command) {
   char buffer[128];
   string result = "";

   // Open pipe to file
   FILE* pipe = popen(command.c_str(), "r");
   if (!pipe) {
      return "popen failed!";
   }

   // read till end of process:
   while (!feof(pipe)) {

      // use buffer to read and add to result
      if (fgets(buffer, 128, pipe) != NULL)
         result += buffer;
   }

   pclose(pipe);
   return result;
}

int main() {
   char buffer[512];
   string des = "", res = "", fil = "", cmd = "shasum ";
   int typ;
   cout << "\nType of sum to check for: sha";
   cin >> typ;
   cout << "\nFile to check: ";
   cin >> fil;
   cout << "\nOriginal hash: ";
   cin >> des;
   des = des.insert(des.length(), fil.insert(0, "  "));
   cmd = cmd.insert(7, fil);
   cmd = cmd.insert(3, to_string(typ));
   res = exec(cmd);
   // Debug
   //cout << des << "\n" << res << "\n";
   if (res == des) {
      cout << "sha" << typ << " sums match.\n";
   } else {
      cout << "sha" << typ << " sums do not match!\n";
   }
}
