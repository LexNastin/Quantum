# sorry, anybody reading this code, well to be precise, the naming
# scheme of a few of the classes, some things just sounded awesome,
# i also really wanted to shove in "shard" somewhere in here, as
# for some reason in my head, it links up with the word "quantum"...
# all weird names, but what can i do, they're amazing! :)

# ah yes, i know why, because to me the word quantum somehow
# is associated with crystal things (probably the fault of slime
# rancher), and crystals can become small shards... oh well :rofl:

# actual display/projection class, will also have main looper, hand
# crafted by me :)

import time
import tkinter as tk
from queue import Queue, Full, Empty

class Shard:
    def __init__(self):
        self.commandQueue = Queue(maxsize=50)
        self.root = tk.Tk()
        self.canvas = tk.Canvas(self.root)

    def enqueue(self, command):
        try:
            self.commandQueue.put_nowait(command)
            return True
        except Full:
            return False

    def update(self):
        begin = time.time_ns()
        try:
            command = self.commandQueue.get_nowait()

            print(command)
        except Empty:
            pass

        # :: add animation into here
        # :: upon exit, exit flask

        self.root.update()
        self.root.update_idletasks()
        ms_taken = (time.time_ns() - begin) / 10**9
        time.sleep(max(0, 1/60-ms_taken))