#!/usr/bin/env python3

# Taken and adapted from here:
# https://github.com/AlexandreSenpai/Bad-Apple/blob/main/run.py

from multiprocessing.pool import ThreadPool
from multiprocessing import cpu_count
from typing import Tuple
import time

import cv2
import tqdm


video_path = './assets/bad-apple.mp4'
code_gen_path = './src/video.rs'
width, height = (64, 48)

def write_frame(frame_information: Tuple[int, cv2.VideoCapture]):
    order, frame = frame_information
    
    y, x, _ = frame.shape
    pixel_row = 0
    
    frame_str = '    [\n'
    for j in range(height):
        frame_str += '        ['
        for i in range(width // 32):
            # Assume width is always divisible by 32
            frame_str += '0b'
            for b in range(32):
                frame_str += '1' if frame[j, 31*i-b].all() == 0 else '0'
            frame_str += ', '

        frame_str += '],\n'
    frame_str += '    ],\n'
    
    return order, frame_str
    

def generate_frames(video: cv2.VideoCapture):
    success = True
    order = 0
    
    while success:
        success, frame = video.read()
        _, bw_frame = cv2.threshold(frame, 128, 255, cv2.THRESH_BINARY)

        if bw_frame is None:
            break
        
        y, x, _ = bw_frame.shape
        bw_frame = cv2.resize(bw_frame, (width, height))
        
        yield order, bw_frame
        
        order += 1

if __name__ == '__main__':
    
    video = cv2.VideoCapture(video_path)
    frame_cnt = int(video.get(cv2.CAP_PROP_FRAME_COUNT))
    
    with ThreadPool(processes=cpu_count()) as pool:
        frames = list(tqdm.tqdm(pool.imap(write_frame, generate_frames(video)), total=frame_cnt))
        pool.close()
        pool.join()
        
    frames = sorted(frames, key=lambda x: x[0])
    
    with open(code_gen_path, 'w') as f:
        f.write('let mut frames: [[u32: graphics::WIDTH]: graphics::HEIGHT] = [\n')
        for _, frame in frames:
            f.write(frame)

        f.write('];\n')
