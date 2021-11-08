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
            RnetExample.Hello("world");
            RnetExample.HelloMany(new string[] { "Alice", "Bob", "Charlie"});
            if (RnetExample.IsEven(42).Item1) {
                Console.WriteLine("42 is even!");
            }
            Console.WriteLine(RnetExample.StrToBytes("Hello, world!").ToString());
            Console.ReadKey(true);
        }
    }
}
