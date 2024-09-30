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

def extract_times(log_data):
    times = []
    for entry in log_data:
        times.append( entry['time'])

    return times


def update(frame, positions, scatter):
    scatter.set_offsets(positions[frame])
    return scatter,

def animate_balls(positions_over_time, times,save_as=None, fps =144):
    fig, ax = plt.subplots()
    
    dim = positions_over_time[0].shape[1]
    
    if dim == 2:
        ax.set_xlim(0, 10)
        ax.set_ylim(0, 10)
    elif dim == 3:
        from mpl_toolkits.mplot3d import Axes3D
        ax = plt.axes(projection='3d')
        ax.set_xlim3d(0, 10)
        ax.set_ylim3d(0, 10)
        ax.set_zlim3d(0, 10)

    scatter = ax.scatter([], [], c='blue')

    total_time = times[-1]-times[0]
    total_len = len(times)
    required_numbers = total_time*fps
    sampling_interval = int(total_len/required_numbers)
    sampled_positions = positions_over_time[::sampling_interval]
    print(f"total_time {total_time}, have {total_len} data.\n sampling_interval {sampling_interval}, so sampled position have {len(sampled_positions)} data")


    ani = FuncAnimation(fig, update, frames=len(sampled_positions),
                        fargs=(sampled_positions, scatter),
                        interval=1000/fps, blit=True)
    
    if save_as:
        ani.save(f'{save_as}.mp4', writer='ffmpeg', fps=fps)


    # plt.show()

def main():
    file_name = os.environ.get('FILE_POSITION')
    log_data = load_log_data(file_name)
    
    positions_over_time = extract_positions(log_data)
    times = extract_times(log_data)

    animate_balls(positions_over_time, times,"balls_moving")

if __name__ == "__main__":
    main()
