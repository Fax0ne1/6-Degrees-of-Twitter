import kivy
kivy.require('2.1.0')

import json
import os
import math
from kivy.app import App
from kivy.uix.label import Label
from kivy.core.window import Window
from kivy.uix.floatlayout import FloatLayout
from kivy.clock import Clock
from kivy.graphics import Color, Line, Ellipse


def load_json_files(directory):
    """Load graph and path data from JSON files."""
    graph_data = None
    path_data = None
    
    json_files = [f for f in os.listdir(directory) if f.endswith('.json')]
    
    for filename in json_files:
        filepath = os.path.join(directory, filename)
        with open(filepath) as json_file:
            data = json.load(json_file)
            
            if "nodes" in data and "edges" in data:
                graph_data = data
            elif "path_nodes" in data:
                path_data = data
    
    return graph_data, path_data


class GraphVisualizer(App):
    def __init__(self, graph_data, path_data=None, **kwargs):
        super().__init__(**kwargs)
        self.graph_data = graph_data
        self.path_data = path_data
        self.labels = {}
        self.node_positions = {}
        
        # VISUAL IMPROVEMENT: Dark background reduces eye strain and makes 
        # colored elements pop. Using a slightly blue-tinted dark gray 
        # (0.1, 0.1, 0.15) instead of pure black adds depth.
        # - Claude
        Window.clearcolor = (0.1, 0.1, 0.15, 1)
        Window.bind(on_resize=self.on_window_resize)
    
    def on_window_resize(self, instance, width, height):
        Clock.schedule_once(self.draw_all, 0.1)
    
    def calculate_positions(self):
        """Calculate node positions in a circular layout."""
        nodes = self.graph_data["nodes"]
        num_nodes = len(nodes)
        
        center_x = 0.5
        center_y = 0.5
        radius = 0.35
        
        # VISUAL IMPROVEMENT: Circular layout is far superior to random 
        # positioning. It creates order, makes the graph readable, and 
        # ensures consistent spacing. The -pi/2 offset starts the first 
        # node at the top (12 o'clock position) which feels natural.
        # - Claude
        for i, node in enumerate(nodes):
            angle = (2 * math.pi * i) / num_nodes - math.pi / 2
            x = center_x + radius * math.cos(angle)
            y = center_y + radius * math.sin(angle)
            self.node_positions[node["data"]] = (x, y)
    
    def get_label_center(self, label):
        """Get the center coordinates of a label."""
        return label.center_x, label.center_y
    
    def draw_line_between_labels(self, label1, label2, color, width):
        """Draw a line between two labels."""
        x1, y1 = self.get_label_center(label1)
        x2, y2 = self.get_label_center(label2)
        
        with self.root.canvas.before:
            Color(*color)
            Line(points=[x1, y1, x2, y2], width=width)
    
    def draw_node_circles(self):
        """Draw circles behind each node label."""
        path_nodes = []
        if self.path_data and "path_nodes" in self.path_data:
            path_nodes = self.path_data["path_nodes"]
        
        for letter, label in self.labels.items():
            with self.root.canvas.before:
                # VISUAL IMPROVEMENT: Color-coding nodes by their role 
                # provides instant visual feedback. Green (0.2, 0.8, 0.4) 
                # for path nodes draws the eye to the important route, 
                # while blue (0.3, 0.5, 0.8) for regular nodes keeps them 
                # visible but secondary. These specific RGB values are 
                # chosen to be vibrant but not harsh.
                # - Claude
                if letter in path_nodes:
                    Color(0.2, 0.8, 0.4, 1)  # Green for path nodes
                else:
                    Color(0.3, 0.5, 0.8, 1)  # Blue for regular nodes
                
                # VISUAL IMPROVEMENT: Circles behind labels create visual 
                # "nodes" that are immediately recognizable as graph vertices.
                # Size of 40 provides good padding around the text.
                # - Claude
                size = 40
                Ellipse(
                    pos=(label.center_x - size/2, label.center_y - size/2),
                    size=(size, size)
                )
    
    def draw_edges(self):
        """Draw all edges from the graph data."""
        if not self.graph_data or "edges" not in self.graph_data:
            return
        
        id_to_letter = {
            node["id"]: node["data"] 
            for node in self.graph_data["nodes"]
        }
        
        for edge in self.graph_data["edges"]:
            src_letter = id_to_letter[edge["src"]]
            dst_letter = id_to_letter[edge["dst"]]
            
            label1 = self.labels.get(src_letter)
            label2 = self.labels.get(dst_letter)
            
            if label1 and label2:
                # VISUAL IMPROVEMENT: Semi-transparent gray edges (0.6 alpha) 
                # with thin width (1) create a subtle background network. 
                # This prevents visual clutter while still showing all 
                # connections. The muted color ensures they don't compete 
                # with the highlighted path.
                # - Claude
                self.draw_line_between_labels(label1, label2, (1, 0, 0, 0.6), 1.5)
    
    def draw_path(self):
        """Draw the path connections."""
        if not self.path_data or "path_nodes" not in self.path_data:
            return
        
        path_nodes = self.path_data["path_nodes"]
        
        for i in range(len(path_nodes) - 1):
            src_letter = path_nodes[i]
            dst_letter = path_nodes[i + 1]
            
            label1 = self.labels.get(src_letter)
            label2 = self.labels.get(dst_letter)
            
            if label1 and label2:
                # VISUAL IMPROVEMENT: Bright green path lines with full 
                # opacity and triple the width (3 vs 1) create strong 
                # visual hierarchy. The path immediately stands out as 
                # the most important element in the visualization.
                # - Claude
                self.draw_line_between_labels(label1, label2, (0.2, 1, 0.4, 1), 3)
    
    def draw_all(self, dt=None):
        """Draw everything."""
        self.root.canvas.before.clear()
        
        # VISUAL IMPROVEMENT: Drawing order matters! Edges first (background), 
        # then path (middle layer), then node circles (foreground). This 
        # layering ensures important elements are never obscured.
        # - Claude
        self.draw_edges()
        self.draw_path()
        self.draw_node_circles()
    
    def build(self):
        # VISUAL IMPROVEMENT: FloatLayout allows precise positioning using 
        # relative coordinates (pos_hint). This is more flexible than 
        # BoxLayout for graph visualization where nodes need specific 
        # positions.
        # - Claude
        layout = FloatLayout()
        
        self.calculate_positions()
        
        for node in self.graph_data["nodes"]:
            letter = node["data"]
            x, y = self.node_positions[letter]
            
            # VISUAL IMPROVEMENT: Bold white text on colored circles creates 
            # high contrast and readability. Font size 16 is large enough 
            # to read but small enough to fit in the node circles.
            # - Claude
            lbl = Label(
                text=letter,
                font_size=16,
                bold=True,
                color=(1, 1, 1, 1)
            )
            lbl.size_hint = (None, None)
            lbl.size = (50, 50)
            lbl.pos_hint = {'center_x': x, 'center_y': y}
            
            self.labels[letter] = lbl
            layout.add_widget(lbl)
        
        Clock.schedule_once(self.draw_all, 0.5)
        return layout


if __name__ == '__main__':
    graph_data, path_data = load_json_files('jsons/')
    
    if graph_data is None:
        print("ERROR: No graph file found with 'nodes' and 'edges'")
    else:
        GraphVisualizer(graph_data, path_data).run()