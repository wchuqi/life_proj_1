import sys
import os

if __name__ == '__main__':
    args = sys.argv[1:]
    comment = args[0]
    os.system('git add .')
    os.system('git commit -m ' + comment)
    os.system('git push')