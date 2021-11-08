using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Data;
using System.Drawing;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Windows.Forms;

namespace RnetExample
{
    public partial class MainForm : Form
    {
        public MainForm()
        {
            InitializeComponent();
        }

        private void runButton_Click(object sender, EventArgs e)
        {
            if (RnetExample.IsEven(42).Item1)
            {
                MessageBox.Show("42 is even!");
            }
            RnetExample.Hello("Diggory");
        }
    }
}
