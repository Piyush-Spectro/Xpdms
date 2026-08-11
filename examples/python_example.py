"""
Example: Using xpTDMS in Python

1. Build & Install:
   maturin develop --features python
   (or pip install .)

2. Usage:
   import xpTDMS
   f = xpTDMS.TdmsFile.open("sample.tdms")
   print(f.group_names())
   data = f.read_channel_f64("Group1", "Channel1")
"""

import xpTDMS

def main():
    print("=== xpTDMS Python Module Usage Example ===")
    print("PyO3 C-Extension Module Loaded:", xpTDMS.__file__)

if __name__ == "__main__":
    main()
