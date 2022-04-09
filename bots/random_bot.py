import base64
import json

while True:
    # Encoded data; ideally, this would be changed to be more
    # beginner-friendly
    encoded_data = input()
    data = json.loads(base64.b64decode(encoded_data))
    
    # Use data to generate data to return
    command = {
        # change the name of the ship | str
        "name": "hi",
        # float
        "thrust": 1.9,
        # float
        "torque": 2.0,
        # bool
        "metal_bullet": False,
        # bool
        "laser_bullet": False,
    }

    # Encode the data to print to stdout
    # Ideally, should be able to simply print all values to be
    # more beginner-friendly
    print(base64.b64encode(bytes(json.dumps(command), 'utf-8')))
