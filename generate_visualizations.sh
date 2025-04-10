#!/bin/bash

# Security block
echo "This is a security block. You must create a virtual environment to run this script."
echo ""
echo "    python3 -m venv venv"
echo "    source venv/bin/activate"
echo ""
echo "If you have already activated the virtual environment, comment this block out."
exit 1

cd gto/

pip install -r requirements.txt

python generate_heatmap.py
python compare_execution_time.py
python compare_mutations.py
