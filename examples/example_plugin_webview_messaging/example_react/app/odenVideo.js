import { useEffect, useState, useRef } from 'react';
import { getOrCreateOdenLayoutClient } from './lib/oden';

function clamp(value, min, max) {
    return Math.min(Math.max(value, min), max);
}

const OdenVideo = ({ name, rotation = '0.0', crop_left = '0.0', crop_right = '0.0',
    crop_top = '0.0', crop_bottom = '0.0', z = '-1', scale_x = '1.0', scale_y = '1.0' }) => {
    const [videoSize, setVideoSize] = useState({ width: 1, height: 1 });
    const videoRef = useRef(null);

    useEffect(() => {
        const layoutClient = getOrCreateOdenLayoutClient();
        if (!layoutClient || !videoRef.current) return;

        layoutClient.registerVideo(name, videoRef.current);

        const updateSize = (videos) => {
            if (name in videos) setVideoSize(videos[name]);
        };
        layoutClient.registerCallback(updateSize);

        return () => {
            layoutClient.unregisterCallback(updateSize);
            layoutClient.unregisterVideo(name);
        };
    }, [name]);

    let float_rotation = parseFloat(rotation);
    let float_crop_left = parseFloat(crop_left);
    let float_crop_right = parseFloat(crop_right);
    let float_crop_top = parseFloat(crop_top);
    let float_crop_bottom = parseFloat(crop_bottom);

    let width = videoSize.width * clamp(1.0 - float_crop_left - float_crop_right, 0.0, 1.0)
    let height = videoSize.height * clamp(1.0 - float_crop_top - float_crop_bottom, 0.0, 1.0)

    let aspectRatio = 0.0;
    if (float_rotation === 90.0 || float_rotation === 270.0) {
        aspectRatio = height / width;
    } else {
        aspectRatio = width / height;
    }

    return (
        <div ref={videoRef} style={{
            aspectRatio: `${aspectRatio}`,
            width: '100%',
            backgroundColor: "transparent",
        }}
            scale-x={scale_x}
            scale-y={scale_y}
            rotation={rotation}
            crop-left={crop_left}
            crop-right={crop_right}
            crop-top={crop_top}
            crop-bottom={crop_bottom}
            z={z}>
        </div >
    );
};

export default OdenVideo;
