import kivy 
kivy.require('2.1.0')
import random
import string
import json
import os
import pandas as pd
from kivy.app import App
from kivy.uix.label import Label
from kivy.core.window import Window
from kivy.uix.boxlayout import BoxLayout
from kivy.clock import Clock
from kivy.graphics import Color, Line

#f = open('graph.json')
#data = json.load(f)

json_files = [pos_json for pos_json in os.listdir('jsons/') if pos_json.endswith('.json')]
jsons_data = pd.DataFrame(columns=['nodes', 'start_user', 'end_user', 'path_nodes'])
for index, js in enumerate(json_files):
    with open(os.path.join('jsons/', js)) as json_file:
        data = json.load(json_file)

print(jsons_data)
class Visuals(App):
    def __init__(self, **kwargs):
        super().__init__(**kwargs)
        Window.bind(on_resize=self.on_window_resize)
    
    def on_window_resize(self, instance, width, height):
        # Clear and redraw lines when window resizes
        Clock.schedule_once(self.drawLine, 0.1)

    def drawConnections()
    
    def drawLine(self, dt=None):
        if "edges" not in data:
            return
        
        self.root.canvas.before.clear()
        
        # Create ID to letter mapping
        id_to_letter = {node["id"]: 
            node["data"] 
            for node in data["nodes"]}
        
        for edge in data["edges"]:
            src_letter = id_to_letter[edge["src"]]
            dst_letter = id_to_letter[edge["dst"]]
            
            label1 = getattr(self, f"l{src_letter}", None)
            label2 = getattr(self, f"l{dst_letter}", None)
            
            if not label1 or not label2:
                continue
            
            x1 = label1.x + label1.width / 2    
            y1 = label1.y + label1.height / 2
            x2 = label2.x + label2.width / 2
            y2 = label2.y + label2.height / 2
            
            with self.root.canvas.before:
                Color(1, .2, .5)  
                Line(points=[x1, y1, x2, y2], width=2)
           
            print(f"y1 = {label1.y} x1 = {label1.x} y2 = {label2.y}, x2 = {label2.x}")  #ignore me, im for debugging
    
    def build(self):
        layout = BoxLayout(orientation='vertical')
        
        # creates a variable for letters
        for node in data["nodes"]:
            letter = node["data"]
            setattr(self, f"l{letter}", Label(text=f"{letter}"))
        
        # creates a widget
        for node in data["nodes"]:
            letter = node["data"]
            lbl = getattr(self, f"l{letter}")  
            layout.add_widget(lbl)
            lbl.size_hint = (None, None)            
            lbl.width = 50
            lbl.height = 30
            lbl.pos_hint = {'x': (random.randrange(250, 700)/1000), 'y': (random.randrange(25, 80)/1000)}
        
        Clock.schedule_once(self.drawLine, 1)
        return layout

if __name__ == '__main__':
    Visuals().run()