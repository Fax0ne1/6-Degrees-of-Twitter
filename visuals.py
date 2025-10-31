import kivy 
import random
import string
import json
kivy.require('2.1.0')

from kivy.app import App
from kivy.uix.boxlayout import BoxLayout
from kivy.uix.label import Label
from kivy.graphics import Color, Rectangle
from kivy.core.window import Window

f = open('input.json')
data = json.load(f)



class Visuals(App):

    def build(self):
        layout = BoxLayout(orientation='vertical')

        for i in data["list"]:
            setattr(self, f"l{i}", Label(text=f"{i}"))
            
        
        for idx, item in enumerate(data["list"]):

            letter = string.ascii_uppercase[idx]     
            widget = getattr(self, f"l{letter}")  
            layout.add_widget(widget)

        for i in data["list"]:

            lbl = getattr(self, f"l{i}")
            lbl.size_hint = (None, None)            
            lbl.width = 100
            lbl.height = 30
            lbl.pos_hint = {'x': random.random(), 'y': random.random()}

        cx, cy = 400, 300 
        radius = 1   

        for i in data["list"]:
            lbl = getattr(self, f"l{i}")

            dx = random.randint(-radius, radius)
            dy = random.randint(-radius, radius)

            lbl.pos = (cx + dx, cy + dy)


        return layout


if __name__ == '__main__':
    Visuals().run()

