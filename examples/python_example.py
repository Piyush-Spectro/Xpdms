"""
Example: Reading TDMS Files in Python using xpTDMS C-Extension

1. Build & Install:
   maturin develop --features python
   (or pip install .)

2. Run Example:
   PYTHONPATH=. python3 examples/python_example.py
"""

import os
import xpTDMS

def main():
    print("==================================================")
    print("🚀 xpTDMS Python Module Example")
    print("==================================================")

    # 1. Test reading Engine Sensor Data
    file1 = "examples/sample_sensor_data.tdms"
    if os.path.exists(file1):
        print(f"\n📂 Opening: {file1}")
        tdms = xpTDMS.TdmsFile.open(file1)

        groups = tdms.group_names()
        print("   Groups:", groups)

        for group in groups:
            channels = tdms.channel_names(group)
            print(f"   Group '{group}' Channels:", channels)

            for channel in channels:
                data = tdms.read_channel_f64(group, channel)
                print(f"   ↳ Channel '{channel}': {len(data)} samples loaded (First 5: {data[:5]})")

    # 2. Test reading Multi-Channel Vibration & Counter Data
    file2 = "examples/sample_multi_channel.tdms"
    if os.path.exists(file2):
        print(f"\n📂 Opening: {file2}")
        tdms2 = xpTDMS.TdmsFile.open(file2)

        groups2 = tdms2.group_names()
        print("   Groups:", groups2)

        vibration = tdms2.read_channel_f32("VibrationSensors", "AccX")
        print(f"   ↳ Channel 'AccX' (f32): {len(vibration)} samples loaded (First 5: {vibration[:5]})")

        pulses = tdms2.read_channel_i32("DigitalCounters", "PulseCount")
        print(f"   ↳ Channel 'PulseCount' (i32): {len(pulses)} samples loaded (First 5: {pulses[:5]})")

    print("\n✅ Python example completed successfully!")

if __name__ == "__main__":
    main()
