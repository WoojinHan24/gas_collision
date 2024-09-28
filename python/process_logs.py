import json
import numpy as np
import matplotlib.pyplot as plt
from matplotlib.animation import FuncAnimation
import os

def load_log_data(file_name):
    with open(file_name, 'r') as file:
        data = json.load(file)
    return data

def extract_positions(log_data):
    positions_over_time = []
    for entry in log_data:
        positions = np.array([ball['coordinates'] for ball in entry['positions']])
        positions_over_time.append(positions)
    return positions_over_time

def update(frame, positions, scatter):
    scatter.set_offsets(positions[frame])
    return scatter,

def animate_balls(positions_over_time, save_as=None):
    fig, ax = plt.subplots()
    
    dim = positions_over_time[0].shape[1]
    
    if dim == 2:
        ax.set_xlim(-10, 10)  # Adjust limits as needed based on your data
        ax.set_ylim(-10, 10)
    elif dim == 3:
        from mpl_toolkits.mplot3d import Axes3D
        ax = plt.axes(projection='3d')
        ax.set_xlim3d(-10, 10)
        ax.set_ylim3d(-10, 10)
        ax.set_zlim3d(-10, 10)

    scatter = ax.scatter([], [], c='blue')
    
    ani = FuncAnimation(fig, update, frames=len(positions_over_time),
                        fargs=(positions_over_time, scatter),
                        interval=100, blit=True)
    
    if save_as:
        # Save as MP4
        ani.save(f'{save_as}.mp4', writer='ffmpeg', fps=1000)


    plt.show()

def main():
    file_name = os.environ.get('FILE_POSITION')
    log_data = load_log_data(file_name)
    
    positions_over_time = extract_positions(log_data)
    
    animate_balls(positions_over_time, "balls_moving")

if __name__ == "__main__":
    main()
