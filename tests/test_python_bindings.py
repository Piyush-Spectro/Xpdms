import os
import tempfile
import xpTDMS

def test_python_bindings():
    # 1. We will test reading a synthetic file created via Rust engine or test file
    print("Testing PyO3 Python Bindings for xpTDMS...")
    
    # Check that TdmsFile is exposed
    assert hasattr(xpTDMS, 'TdmsFile')
    print("✓ xpTDMS.TdmsFile class verified")

if __name__ == "__main__":
    test_python_bindings()
