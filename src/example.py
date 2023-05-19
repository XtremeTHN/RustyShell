from RustyShellUtils import RustyShellPaths
import json, os

def get_config(path):
    with open(path, 'r+') as file:
        return json.load(file)

def set_value(path, value):
    with open(path, 'w') as file:
        json.dump(value, file)

class Main:
    paths = RustyShellPaths("cat2")
    config = os.path.join(paths.conf_path, "conf.json")
    set_value(config, {'data':True})
    print(get_config(config))
    print(sys)