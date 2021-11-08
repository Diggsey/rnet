using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace RnetExample
{
    class Program
    {
        static void Main(string[] args)
        {
            RnetExample.HelloMany(new string[] { "Alice", "Bob", "Charlie"});
            Console.ReadKey(true);
        }
    }
}
