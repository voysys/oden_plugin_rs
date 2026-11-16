/*
 * Copyright (c) 2014-2026 Voysys AB. All rights reserved.
 *
 * Permission is granted to use, copy and redistribute this file, modified or
 * unmodified, for the purpose of building plugins for Oden.
 *
 * THIS FILE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED. IN NO EVENT SHALL VOYSYS AB BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY ARISING FROM THE USE OF THIS FILE.
 */

#pragma once

#ifdef __cplusplus
extern "C" {
#endif

#include <stdbool.h>
#include <stdint.h>

#define PLUGIN_ENTITY_ID_SIZE   36
#define PLUGIN_ENTITY_NAME_SIZE 31

#define PLUGIN_VIDEO_CAPTURE_ID_SIZE   36
#define PLUGIN_VIDEO_CAPTURE_NAME_SIZE 31
#define ODEN_NETWORK_DEVICE_NAME_SIZE  63

#define ARRAY_COUNT(arr) (sizeof(arr) / sizeof((arr)[0]))

#ifndef DEPRECATED
#if defined(__GNUC__) || defined(__clang__)
#define DEPRECATED(warning) __attribute__((deprecated(warning)))
#elif defined(_MSC_VER)
#define DEPRECATED(warning) __declspec(deprecated(warning))
#else
#pragma message("WARNING: You need to implement DEPRECATED for this compiler")
#define DEPRECATED(...)
#endif
#endif

#define PLUGIN_MOUSE_BUTTON_1      0
#define PLUGIN_MOUSE_BUTTON_2      1
#define PLUGIN_MOUSE_BUTTON_3      2
#define PLUGIN_MOUSE_BUTTON_4      3
#define PLUGIN_MOUSE_BUTTON_5      4
#define PLUGIN_MOUSE_BUTTON_6      5
#define PLUGIN_MOUSE_BUTTON_7      6
#define PLUGIN_MOUSE_BUTTON_8      7
#define PLUGIN_MOUSE_BUTTON_LAST   PLUGIN_MOUSE_BUTTON_8
#define PLUGIN_MOUSE_BUTTON_LEFT   PLUGIN_MOUSE_BUTTON_1
#define PLUGIN_MOUSE_BUTTON_RIGHT  PLUGIN_MOUSE_BUTTON_2
#define PLUGIN_MOUSE_BUTTON_MIDDLE PLUGIN_MOUSE_BUTTON_3

#define PLUGIN_JOYSTICK_1    0
#define PLUGIN_JOYSTICK_2    1
#define PLUGIN_JOYSTICK_3    2
#define PLUGIN_JOYSTICK_4    3
#define PLUGIN_JOYSTICK_5    4
#define PLUGIN_JOYSTICK_6    5
#define PLUGIN_JOYSTICK_7    6
#define PLUGIN_JOYSTICK_8    7
#define PLUGIN_JOYSTICK_9    8
#define PLUGIN_JOYSTICK_10   9
#define PLUGIN_JOYSTICK_11   10
#define PLUGIN_JOYSTICK_12   11
#define PLUGIN_JOYSTICK_13   12
#define PLUGIN_JOYSTICK_14   13
#define PLUGIN_JOYSTICK_15   14
#define PLUGIN_JOYSTICK_16   15
#define PLUGIN_JOYSTICK_LAST PLUGIN_JOYSTICK_16

#define PLUGIN_JOYSTICK_MAX_AXES    128
#define PLUGIN_GAMEPAD_BUTTON_COUNT 26

#define PLUGIN_KEY_UNKNOWN       (-1)
#define PLUGIN_KEY_SPACE         32
#define PLUGIN_KEY_APOSTROPHE    39
#define PLUGIN_KEY_COMMA         44
#define PLUGIN_KEY_MINUS         45
#define PLUGIN_KEY_PERIOD        46
#define PLUGIN_KEY_SLASH         47
#define PLUGIN_KEY_0             48
#define PLUGIN_KEY_1             49
#define PLUGIN_KEY_2             50
#define PLUGIN_KEY_3             51
#define PLUGIN_KEY_4             52
#define PLUGIN_KEY_5             53
#define PLUGIN_KEY_6             54
#define PLUGIN_KEY_7             55
#define PLUGIN_KEY_8             56
#define PLUGIN_KEY_9             57
#define PLUGIN_KEY_SEMICOLON     59
#define PLUGIN_KEY_EQUAL         61
#define PLUGIN_KEY_A             65
#define PLUGIN_KEY_B             66
#define PLUGIN_KEY_C             67
#define PLUGIN_KEY_D             68
#define PLUGIN_KEY_E             69
#define PLUGIN_KEY_F             70
#define PLUGIN_KEY_G             71
#define PLUGIN_KEY_H             72
#define PLUGIN_KEY_I             73
#define PLUGIN_KEY_J             74
#define PLUGIN_KEY_K             75
#define PLUGIN_KEY_L             76
#define PLUGIN_KEY_M             77
#define PLUGIN_KEY_N             78
#define PLUGIN_KEY_O             79
#define PLUGIN_KEY_P             80
#define PLUGIN_KEY_Q             81
#define PLUGIN_KEY_R             82
#define PLUGIN_KEY_S             83
#define PLUGIN_KEY_T             84
#define PLUGIN_KEY_U             85
#define PLUGIN_KEY_V             86
#define PLUGIN_KEY_W             87
#define PLUGIN_KEY_X             88
#define PLUGIN_KEY_Y             89
#define PLUGIN_KEY_Z             90
#define PLUGIN_KEY_LEFT_BRACKET  91
#define PLUGIN_KEY_BACKSLASH     92
#define PLUGIN_KEY_RIGHT_BRACKET 93
#define PLUGIN_KEY_GRAVE_ACCENT  96
#define PLUGIN_KEY_WORLD_1       161
#define PLUGIN_KEY_WORLD_2       162
#define PLUGIN_KEY_ESCAPE        256
#define PLUGIN_KEY_ENTER         257
#define PLUGIN_KEY_TAB           258
#define PLUGIN_KEY_BACKSPACE     259
#define PLUGIN_KEY_INSERT        260
#define PLUGIN_KEY_DELETE        261
#define PLUGIN_KEY_RIGHT         262
#define PLUGIN_KEY_LEFT          263
#define PLUGIN_KEY_DOWN          264
#define PLUGIN_KEY_UP            265
#define PLUGIN_KEY_PAGE_UP       266
#define PLUGIN_KEY_PAGE_DOWN     267
#define PLUGIN_KEY_HOME          268
#define PLUGIN_KEY_END           269
#define PLUGIN_KEY_CAPS_LOCK     280
#define PLUGIN_KEY_SCROLL_LOCK   281
#define PLUGIN_KEY_NUM_LOCK      282
#define PLUGIN_KEY_PRINT_SCREEN  283
#define PLUGIN_KEY_PAUSE         284
#define PLUGIN_KEY_F1            290
#define PLUGIN_KEY_F2            291
#define PLUGIN_KEY_F3            292
#define PLUGIN_KEY_F4            293
#define PLUGIN_KEY_F5            294
#define PLUGIN_KEY_F6            295
#define PLUGIN_KEY_F7            296
#define PLUGIN_KEY_F8            297
#define PLUGIN_KEY_F9            298
#define PLUGIN_KEY_F10           299
#define PLUGIN_KEY_F11           300
#define PLUGIN_KEY_F12           301
#define PLUGIN_KEY_F13           302
#define PLUGIN_KEY_F14           303
#define PLUGIN_KEY_F15           304
#define PLUGIN_KEY_F16           305
#define PLUGIN_KEY_F17           306
#define PLUGIN_KEY_F18           307
#define PLUGIN_KEY_F19           308
#define PLUGIN_KEY_F20           309
#define PLUGIN_KEY_F21           310
#define PLUGIN_KEY_F22           311
#define PLUGIN_KEY_F23           312
#define PLUGIN_KEY_F24           313
#define PLUGIN_KEY_F25           314
#define PLUGIN_KEY_KP_0          320
#define PLUGIN_KEY_KP_1          321
#define PLUGIN_KEY_KP_2          322
#define PLUGIN_KEY_KP_3          323
#define PLUGIN_KEY_KP_4          324
#define PLUGIN_KEY_KP_5          325
#define PLUGIN_KEY_KP_6          326
#define PLUGIN_KEY_KP_7          327
#define PLUGIN_KEY_KP_8          328
#define PLUGIN_KEY_KP_9          329
#define PLUGIN_KEY_KP_DECIMAL    330
#define PLUGIN_KEY_KP_DIVIDE     331
#define PLUGIN_KEY_KP_MULTIPLY   332
#define PLUGIN_KEY_KP_SUBTRACT   333
#define PLUGIN_KEY_KP_ADD        334
#define PLUGIN_KEY_KP_ENTER      335
#define PLUGIN_KEY_KP_EQUAL      336
#define PLUGIN_KEY_LEFT_SHIFT    340
#define PLUGIN_KEY_LEFT_CONTROL  341
#define PLUGIN_KEY_LEFT_ALT      342
#define PLUGIN_KEY_LEFT_SUPER    343
#define PLUGIN_KEY_RIGHT_SHIFT   344
#define PLUGIN_KEY_RIGHT_CONTROL 345
#define PLUGIN_KEY_RIGHT_ALT     346
#define PLUGIN_KEY_RIGHT_SUPER   347
#define PLUGIN_KEY_MENU          348
#define PLUGIN_KEY_LAST          PLUGIN_KEY_MENU

#define PLUGIN_KEY_ACTION_RELEASE 0
#define PLUGIN_KEY_ACTION_PRESS   1
#define PLUGIN_KEY_ACTION_REPEAT  2

#define PLUGIN_MOD_CONTROL    0x0002
#define PLUGIN_MOD_SHIFT      0x0001
#define PLUGIN_MOD_ALT        0x0004
#define PLUGIN_MOD_SUPER      0x0008
#define PLUGIN_MOD_FROM_TOUCH 0x0100

// Fixups

#define imDrawIsMouseOverLastElemennt imDrawIsMouseOverLastElement
#define setFpsCameraAngle             setFpsCameraAnglesAndPosition

// Faults

#define ODEN_FAULT_OCULUS_DISCONNECTED   0
#define ODEN_FAULT_HMD_DISCONNECTED      0
#define ODEN_FAULT_HMD_REMOVED_FROM_HEAD 1

// Types

typedef struct OdenUuid_s {
    uint8_t uuid[16];
} OdenUuid;

typedef struct OdenVec2_s {
#ifdef RUST_BINDGEN
    float d[2];
#else
    union {
        float d[2];
        struct {
            float x;
            float y;
        };
    };
#endif
} OdenVec2;

typedef struct OdenVec3_s {
#ifdef RUST_BINDGEN
    float d[3];
#else
    union {
        float d[3];
        struct {
            float x;
            float y;
            float z;
        };
    };
#endif
} OdenVec3;

typedef struct OdenVec4_s {
#ifdef RUST_BINDGEN
    float d[4];
#else
    union {
        float d[4];
        struct {
            float x;
            float y;
            float z;
            float w;
        };
    };
#endif
} OdenVec4;

typedef struct OdenVec2i_s {
#ifdef RUST_BINDGEN
    int32_t d[2];
#else
    union {
        int32_t d[2];
        struct {
            int32_t x;
            int32_t y;
        };
    };
#endif
} OdenVec2i;

typedef struct OdenMatrix4_s {
    float d[4][4];
} OdenMatrix4;

typedef struct OdenQuat_s {
#ifdef RUST_BINDGEN
    float d[4];
#else
    union {
        float d[4];
        struct {
            float w;
            float x;
            float y;
            float z;
        };
    };
#endif
} OdenQuat;

typedef struct OdenRay_s {
    OdenVec3 origin;
    OdenVec3 direction;
} OdenRay;

typedef struct OdenCameraCalibration_s {
    float k[5];
    float offset[2];
    float position[3];
    float rotation[4]; // quaternion in w, x, y, z order
    float maxTheta;
    float globalPosition[3];
    float globalRotation[4]; // quaternion in w, x, y, z order
} OdenCameraCalibration;

typedef struct OdenCameraCropData_s {
    float left;
    float right;
    float up;
    float down;
    float circular;
} OdenCameraCropData;

typedef enum OdenPluginVideoFormat_e {
    OdenPluginVideoFormatRgb = 0,
    OdenPluginVideoFormatBgr = 1,
    OdenPluginVideoFormatYuv = 2,
    OdenPluginVideoFormatNv12 = 3,
    OdenPluginVideoFormatYuyv = 4,
    OdenPluginVideoFormatUyvy = 5,
    OdenPluginVideoFormatRaw = 6,
    OdenPluginVideoFormatMono16 = 7,
    OdenPluginVideoFormatRaw16 = 8,
    OdenPluginVideoFormatMono8 = 9,
    OdenPluginVideoFormatRaw10 = 10,
    OdenPluginVideoFormatRgba = 11,
    OdenPluginVideoFormatV210 = 12,
    OdenPluginVideoFormatBgra = 13,
    OdenPluginVideoFormatArgb = 14,
    OdenPluginVideoFormatUyyvyy = 15,
    OdenPluginVideoFormatInvalid = -1,
} OdenPluginVideoFormat;

typedef struct OdenPluginVideoFrame_s {
    // Frame info
    int32_t width;
    int32_t height;
    int64_t timestamp;
    OdenPluginVideoFormat format;

    // Mapped frame
    uint8_t * data[3];
    int32_t size[3];
    int32_t stride[3];

} OdenPluginVideoFrame;

typedef enum OdenPluginCodec_e {
    OdenPluginCodecH264 = 0,
    OdenPluginCodecH265 = 1,
    OdenPluginCodecMJPEG = 2,
    OdenPluginCodecMaxEnum = 0x7FFFFFFF,
} OdenPluginCodec;

typedef struct OdenTextureStreamingConfig_s {
    int32_t width;
    int32_t height;
    int32_t frame_rate;
    int32_t bitrate_kbps;
    OdenPluginCodec codec;
    const char * output_pipeline;
    const char * entity_id;
    int32_t stream;
    int32_t reserved_i32[8];
    void * reserved_ptr[4];
} OdenTextureStreamingConfig;

typedef enum OdenTextureStreamingError_e {
    OdenTextureStreamingErrorOk = 0,
    OdenTextureStreamingErrorUnknown = 1,
    OdenTextureStreamingErrorUnsupportedApplication = 2,
    OdenTextureStreamingErrorArgumentIsNull = 3,
    OdenTextureStreamingErrorUnsupportedCodec = 4,
    OdenTextureStreamingErrorOutputPipelineRequired = 5,
    OdenTextureStreamingErrorInvalidNumericField = 6,
    OdenTextureStreamingErrorVideoCaptureNotFound = 7,
    OdenTextureStreamingErrorNotConfigured = 8,
    OdenTextureStreamingErrorMaxEnum = 0x7FFFFFFF,
} OdenTextureStreamingError;

typedef struct OdenAsyncComChannelMessageId_s {
    const char * messageId;
    int32_t dataSize;
} OdenAsyncComChannelMessageId;

typedef struct OdenAsyncComChannelVarSizeMsgId_s {
    const char * messageId;
} OdenAsyncComChannelVarSizeMsgId;

typedef enum OdenAsyncComChannelReceiveError_e {
    OdenAsyncComChannelReceiveErrorTimeout = -1,
    OdenAsyncComChannelReceiveErrorBufferTooSmallForMessage = -2,
    OdenAsyncComChannelReceiveErrorUnknown = -3,
    OdenAsyncComChannelReceiveErrorGotMessageWithWrongSize = -4,
} OdenAsyncComChannelReceiveError;

typedef enum OdenRegulatorMode_e {
    OdenRegulatorModeAuto = 0,
    OdenRegulatorModeLow,
    OdenRegulatorModeMedium,
    OdenRegulatorModeHigh,
} OdenRegulatorMode;

typedef enum OdenApplicationType_e {
    Player = 0,
    Streamer = 2,
} OdenApplicationType;

typedef enum OdenEventType_e {
    OdenEventTypeKey,
    OdenEventTypeMouseButton,
    OdenEventTypeScroll,
    OdenEventTypeChar,
    OdenEventTypeFramebufferResize,
    OdenEventTypeWindowResize,
    OdenEventTypeWindowPos,
    OdenEventTypeWindowFocus,
    OdenEventTypeTouchInput,
    OdenEventTypeShutdown,
} OdenEventType;

typedef enum OdenLogLevel_e {
    OdenLogTrace = 0,
    OdenLogDebug = 1,
    OdenLogInfo = 2,
    OdenLogWarning = 3,
    OdenLogError = 4,
    OdenLogCritical = 5,
    OdenLogNone = 6,
} OdenLogLevel;

typedef struct OdenKeyEvent_s {
    int32_t key;
    int32_t scancode;
    int32_t actions;
    int32_t mods;
} OdenKeyEvent;

typedef struct OdenMouseButtonEvent_s {
    int32_t key;
    int32_t actions;
    int32_t mods;
} OdenMouseButtonEvent;

typedef struct OdenScrollEvent_s {
    double x;
    double y;
} OdenScrollEvent;

typedef struct OdenCharEvent_s {
    uint32_t c;
} OdenCharEvent;

typedef struct OdenFramebufferResizeEvent_s {
    OdenVec2i framebufferSize;
} OdenFramebufferResizeEvent;

typedef struct OdenWindowResizeEvent_s {
    OdenVec2i windowSize;
} OdenWindowResizeEvent;

typedef struct OdenWindowPosEvent_s {
    OdenVec2i windowPosition;
} OdenWindowPosEvent;

typedef struct OdenWindowFocusEvent_s {
    bool isFocused;
} OdenWindowFocusEvent;

typedef enum OdenTouchInputAction_e {
    Press = 0,
    Move,
    Release,
} OdenTouchInputAction;

typedef struct OdenTouchInputEvent_s {
    int32_t touchId;
    OdenTouchInputAction action;
    OdenVec2 pos;
} OdenTouchInputEvent;

typedef struct OdenWindowEvent_s {
    OdenEventType type;
    union {
        OdenKeyEvent k;
        OdenMouseButtonEvent mb;
        OdenScrollEvent s;
        OdenCharEvent c;
        OdenFramebufferResizeEvent fr;
        OdenWindowResizeEvent wr;
        OdenWindowPosEvent wp;
        OdenWindowFocusEvent wf;
        OdenTouchInputEvent t;
    };
} OdenWindowEvent;

typedef struct OdenRayToVideoIntersect_s {
    bool intersectedVideo;
    OdenVec2 uvCoords;
    OdenVec2 pixelCoords;
    OdenVec3 intersectCoordinate;
} OdenRayToVideoIntersect;

typedef enum OdenTensorMode_e {
    OdenTensorModeInput = 0,
    OdenTensorModeOutput,
} OdenTensorMode;

typedef enum OdenTensorFormat_e {
    OdenTensorFormatLinear = 0,
} OdenTensorFormat;

#define PLUGIN_TENSOR_SHAPE_MAX_DIMS 8
typedef struct OdenTensorShape_s {
    // The rank (number of dimensions)
    int32_t nbDims;
    // The extent of each dimension
    int32_t d[PLUGIN_TENSOR_SHAPE_MAX_DIMS];
} OdenTensorShape;

typedef enum OdenTensorDataType_e {
    OdenTensorDataTypeFp32 = 0,
} OdenTensorDataType;

typedef enum OdenNeuralNetworkError_e {
    OdenNeuralNetworkErrorOk = 0,
    OdenNeuralNetworkErrorUnknown = 1,
    OdenNeuralNetworkErrorNotSupported = 2,
    OdenNeuralNetworkErrorFailedToLoadNvInfer = 3,
    OdenNeuralNetworkErrorInvalidUuid = 4,
    OdenNeuralNetworkErrorInvalidPath = 5,
    OdenNeuralNetworkErrorInvalidTensorName = 6,
    OdenNeuralNetworkErrorInvalidEntityId = 7,
    OdenNeuralNetworkErrorInvalidTensorData = 8,
    OdenNeuralNetworkErrorNetworkNotFound = 9,
    OdenNeuralNetworkErrorIsLoading = 10,
    OdenNeuralNetworkErrorNotLoaded = 11,
    OdenNeuralNetworkErrorArgumentIsNull = 12,
    OdenNeuralNetworkErrorMaxEnum = 0x7FFFFFFF,
} OdenNeuralNetworkError;

// Valid for current frame only
typedef struct OdenTensorData_s {
    const char * name;
    OdenTensorFormat format;
    OdenTensorShape shape;
    OdenTensorDataType dataType;

    const uint8_t * data; // is null for input tensors
    uint64_t dataSize;

    int64_t timestamp;
    int32_t frameId;

    uint8_t reserved[16];
} OdenTensorData;

typedef struct OdenTensorInput_s {
    const char * name;
    OdenUuid videoStreamUuid;
} OdenTensorInput;

typedef struct OdenNeuralNetworkInfo_s {
    OdenUuid uuid;
    const char * networkPath;
    bool manuallyTrigger;

    OdenTensorInput inputs[8];
    uint16_t tensorInputsCount;

    uint8_t reserved[16];
} OdenNeuralNetworkInfo;

typedef struct OdenInferenceSettings_s {
    OdenNeuralNetworkInfo neuralNetworks[32];
    uint16_t neuralNetworkCount;

    uint8_t reserved[16];
} OdenInferenceSettings;

typedef enum OdenPluginMouseCursor_e {
    OdenPluginMouseCursorArrow = 0,
    OdenPluginMouseCursorTextInput,
    OdenPluginMouseCursorPointingHand,
    OdenPluginMouseCursorResizeNS,
    OdenPluginMouseCursorResizeEW,
    OdenPluginMouseCursorResizeNESW,
    OdenPluginMouseCursorResizeNWSE,
    OdenPluginMouseCursorMaxEnum = 0x7FFFFFFF
} OdenPluginMouseCursor;

// Global functions

typedef void (*OdenLogInfoFunc)(const char * message, ...);
typedef void (*OdenLogWarningFunc)(const char * message, ...);
typedef void (*OdenLogErrorFunc)(const char * message, ...);
typedef void (*OdenLogRawFunc)(
    OdenLogLevel level,
    const char * file,
    const char * function,
    int line,
    const char * module,
    const char * message,
    ...);

typedef bool (*OdenCopyFrameFunc)(OdenPluginVideoFrame src, OdenPluginVideoFrame dst);

typedef bool (*OdenConvertFrameToPackedRgbFunc)(
    OdenPluginVideoFrame frame,
    int32_t outputWidth,
    int32_t outputHeight,
    uint8_t * rgbOutputBuffer,
    int32_t rgbOutputBufferSize);

typedef void (*OdenConvertLidarDataToPointCloudFunc)(void);

typedef OdenRay (*OdenProjectCameraFunc)(
    OdenCameraCalibration calibration, int32_t imageWidth, int32_t imageHeight, OdenVec2 pixelCoords);

typedef bool (*OdenClosestPointBetweenRaysFunc)(OdenRay ray1, OdenRay ray2, OdenVec3 * result);

typedef void (*OdenRectifyImageFunc)(
    OdenCameraCalibration calibration,
    const OdenCameraCropData * cropData,
    OdenPluginVideoFrame srcPluginFrame,
    const OdenMatrix4 entityWorldMatrix,
    const OdenVec3 center,
    const OdenVec3 up,
    const float outputHorizontalFov,
    int32_t outputWidth,
    int32_t outputHeight,
    uint8_t * rgbOutputBuffer,
    int32_t rgbOutputBufferSize);

typedef OdenRay (*OdenProjectRectifiedImageFunc)(
    OdenCameraCalibration calibration,
    const OdenMatrix4 entityWorldMatrix,
    const OdenVec3 center,
    const OdenVec3 up,
    const float outputHorizontalFov,
    int32_t imageWidth,
    int32_t imageHeight,
    OdenVec2 pixelCoords);

typedef void (*OdenMultMatrixFunc)(OdenMatrix4 a, OdenMatrix4 b, OdenMatrix4 * out);
typedef OdenVec3 (*OdenGetRotationXyzFunc)(OdenMatrix4 mat);

typedef void (*OdenAsyncComChannelAddMessageToSendQueueFunc)(
    const char * messageId, const uint8_t * data, int32_t dataSize);
typedef void (*OdenAsyncComChannelFlushMessageQueueFunc)(void);

typedef int32_t (*OdenAsyncComChannelReceiveFunc)(
    const OdenAsyncComChannelMessageId * messageIds,
    int32_t messageIdCount,
    uint8_t * buffer,
    int32_t bufferSize,
    int32_t timeoutMs);

typedef int32_t (*OdenAsyncComChannelReceiveVarSizeMsgFunc)(
    const OdenAsyncComChannelVarSizeMsgId * messageIds,
    int32_t messageIdCount,
    uint8_t * buffer,
    int32_t bufferSize,
    int32_t * receivedSize,
    int32_t timeoutMs);

#define ODEN_JOYSTICK_POSE_AXIS         64
#define ODEN_JOYSTICK_REFERENCE_AXIS    80
#define ODEN_JOYSTICK_POSE_FLAGS_AXIS   96
#define ODEN_JOYSTICK_POSE_ORIENTATION  1
#define ODEN_JOYSTICK_POSE_POSITION     2
#define ODEN_JOYSTICK_POSE_VALID        4
#define ODEN_JOYSTICK_POSE_CALIBRATED   8
#define ODEN_JOYSTICK_POSE_RELATIVE_YAW 16
typedef struct OdenJoystickState_s {
    int32_t id;
    char name[128];
    int32_t axesCount;
    float axes[PLUGIN_JOYSTICK_MAX_AXES];
    int32_t buttonsCount;
    unsigned char buttons[PLUGIN_JOYSTICK_MAX_AXES];
} OdenJoystickState;

typedef enum OdenPowerState_e {
    OdenPowerStateUnknown = 0,
    OdenPowerStateOnBattery = 1,
    OdenPowerStateNoBattery = 2,
    OdenPowerStateCharging = 3,
    OdenPowerStateCharged = 4,
    OdenPowerStateMaxEnum = 0x7FFFFFFF,
} OdenPowerState;

typedef struct OdenGamepadState_s {
    int32_t id;
    OdenUuid uuid;
    unsigned char buttons[15];
    float axes[6];
} OdenGamepadState;

typedef struct OdenGamepadStateV2_s {
    int32_t id;
    OdenUuid uuid;
    unsigned char buttons[PLUGIN_GAMEPAD_BUTTON_COUNT];
    float axes[6];
} OdenGamepadStateV2;

typedef bool (*OdenIsJoystickPresentFunc)(int32_t joystickId);
typedef bool (*OdenGetJoystickStateFunc)(int32_t joystickId, OdenJoystickState * joystickStateOut);
typedef bool (*OdenGetGamepadStateFunc)(int32_t joystickId, OdenGamepadState * gamepadStateOut);
typedef bool (*OdenGetGamepadStateV2Func)(int32_t joystickId, OdenGamepadStateV2 * gamepadStateOut);
typedef bool (*OdenSetJoystickMotionEnabledFunc)(int32_t joystickId, bool enabled);
typedef bool (*OdenGetGamepadFunc)(OdenGamepadState * gamepadStateOut);
typedef bool (*OdenAddGamepadMappingFunc)(const char * mapping);
typedef bool (*OdenGetPowerInfoFunc)(int32_t * seconds, int32_t * percent, OdenPowerState * state);

#define ODEN_HAND_JOINT_PALM                0
#define ODEN_HAND_JOINT_WRIST               1
#define ODEN_HAND_JOINT_THUMB_METACARPAL    2
#define ODEN_HAND_JOINT_THUMB_PROXIMAL      3
#define ODEN_HAND_JOINT_THUMB_DISTAL        4
#define ODEN_HAND_JOINT_THUMB_TIP           5
#define ODEN_HAND_JOINT_INDEX_METACARPAL    6
#define ODEN_HAND_JOINT_INDEX_PROXIMAL      7
#define ODEN_HAND_JOINT_INDEX_INTERMEDIATE  8
#define ODEN_HAND_JOINT_INDEX_DISTAL        9
#define ODEN_HAND_JOINT_INDEX_TIP           10
#define ODEN_HAND_JOINT_MIDDLE_METACARPAL   11
#define ODEN_HAND_JOINT_MIDDLE_PROXIMAL     12
#define ODEN_HAND_JOINT_MIDDLE_INTERMEDIATE 13
#define ODEN_HAND_JOINT_MIDDLE_DISTAL       14
#define ODEN_HAND_JOINT_MIDDLE_TIP          15
#define ODEN_HAND_JOINT_RING_METACARPAL     16
#define ODEN_HAND_JOINT_RING_PROXIMAL       17
#define ODEN_HAND_JOINT_RING_INTERMEDIATE   18
#define ODEN_HAND_JOINT_RING_DISTAL         19
#define ODEN_HAND_JOINT_RING_TIP            20
#define ODEN_HAND_JOINT_PINKY_METACARPAL    21
#define ODEN_HAND_JOINT_PINKY_PROXIMAL      22
#define ODEN_HAND_JOINT_PINKY_INTERMEDIATE  23
#define ODEN_HAND_JOINT_PINKY_DISTAL        24
#define ODEN_HAND_JOINT_PINKY_TIP           25
#define ODEN_HAND_JOINT_COUNT               26

#define ODEN_HAND_FLAG_ACTIVE 1
#define ODEN_HAND_FLAG_LEFT   2
#define ODEN_HAND_FLAG_RIGHT  4
#define ODEN_HAND_FLAG_LOCAL  8

typedef struct OdenHandTrackingState_s {
    uint8_t flags;
    OdenVec3 wrist_position;
    OdenQuat wrist_rotation;
    OdenVec3 joint_positions[ODEN_HAND_JOINT_COUNT];
} OdenHandTrackingState;

typedef bool (*OdenGetHandTrackingStateFunc)(int32_t hand, OdenHandTrackingState * stateOut);
typedef void (*OdenHandToWorldFunc)(OdenHandTrackingState * state);
typedef void (*OdenHandToLocalFunc)(OdenHandTrackingState * state);

typedef struct OdenPluginGlobalFunctions_s {

    // Log
    OdenLogInfoFunc logInfo;
    OdenLogWarningFunc logWarning;
    OdenLogErrorFunc logError;
    OdenLogRawFunc logRaw;
    void * logReserved[15];

    // Frame
    OdenCopyFrameFunc copyFrame;
    OdenConvertFrameToPackedRgbFunc convertFrameToRgb;
    void * frameReserved[16];

    // Lidar
    OdenConvertLidarDataToPointCloudFunc convertLidarDataToPointCloud;
    void * lidarReserved[16];

    // Math
    OdenProjectCameraFunc projectCamera;
    OdenClosestPointBetweenRaysFunc closestPointBetweenRays;
    OdenMultMatrixFunc multMatrix;
    OdenGetRotationXyzFunc getRotationXyz;
    void * mathReserved[64];

    // Rectification / Dewarping
    OdenRectifyImageFunc rectifyImage;
    OdenProjectRectifiedImageFunc projectRectifiedImage;
    void * rectificationReserved[16];

    // Async com channels
    OdenAsyncComChannelAddMessageToSendQueueFunc asyncComChannelAddMessageToSendQueue;
    OdenAsyncComChannelFlushMessageQueueFunc asyncComChannelFlushMessageQueue;
    OdenAsyncComChannelReceiveFunc asyncComChannelReceive;

    OdenAsyncComChannelReceiveVarSizeMsgFunc asyncComChannelReceiveVarSizeMsg;

    OdenIsJoystickPresentFunc isJoystickPresent;
    OdenGetJoystickStateFunc getJoystickState;
    OdenGetGamepadStateFunc getGamepadState;
    OdenGetGamepadFunc getGamepad;
    OdenAddGamepadMappingFunc addGamepadMapping;
    OdenGetPowerInfoFunc getPowerInfo;
    OdenGetGamepadStateV2Func getGamepadStateV2;
    OdenGetHandTrackingStateFunc getHandTrackingState;
    OdenHandToWorldFunc handToWorld;
    OdenHandToLocalFunc handToLocal;
    OdenSetJoystickMotionEnabledFunc setJoystickMotionEnabled;

    void * reserved[224];

} OdenPluginGlobalFunctions;

extern const OdenPluginGlobalFunctions * oden;

// Settings functions

typedef bool (*OdenSettingsReadIntFunc)(const char * key, int32_t * result);
typedef bool (*OdenSettingsReadFloatFunc)(const char * key, float * result);
typedef bool (*OdenSettingsReadStringFunc)(const char * key, char * result, int32_t resultSize);
typedef void (*OdenSettingsWriteIntFunc)(const char * key, int32_t value);
typedef void (*OdenSettingsWriteFloatFunc)(const char * key, float value);
typedef void (*OdenSettingsWriteStringFunc)(const char * key, const char * value);
typedef bool (*OdenSettingsGetProjectPathFunc)(char * projectPath, int32_t * projectPathCount);
typedef bool (*OdenSettingsGetProjectPluginsPathFunc)(
    char * projectPluginsPath, int32_t * projectPluginsPathCount);
typedef OdenApplicationType (*OdenGetApplicationTypeFunc)(void);
typedef void (*OdenSetHasCrashedFunc)(void);
typedef void (*OdenSetUiDisabledFunc)(bool);
typedef bool (*OdenSettingsReadSceneStringFunc)(const char * key, char * result, int32_t resultSize);
typedef bool (*OdenSettingsWriteSceneStringFunc)(const char * key, const char * value);
typedef void (*OdenExitApplicationFunc)(bool forceExit);
typedef void (*OdenPreventCloseFunc)(void);
typedef bool (*OdenCloseWasDeniedFunc)(void);
typedef bool (*OdenSettingsReadStringExFunc)(const char * key, char * result, int32_t * resultSize);
typedef bool (*OdenSettingsGetProjectFilePathFunc)(char * projectFilePath, int32_t * projectFilePathLen);

// Update functions

typedef void (*OdenUpdateRecordingPushDataFunc)(const uint8_t * data, int32_t dataSize);
typedef void (*OdenUpdateResetViewFunc)(void);
typedef void (*OdenUpdateSetFpsCameraAnglesAndPositionFunc)(
    float x /* yaw, radians */, float y /* pitch, radians */, OdenVec3 pos);
typedef void (*OdenUpdateGetMouseRayFunc)(OdenVec3 * origin, OdenVec3 * direction);
typedef void (*OdenUpdateGetCameraForwardRayFunc)(OdenVec3 * origin, OdenVec3 * direction);
typedef bool (*OdenUpdateIsMousePressedSinceLastSwapFunc)(void);
typedef void (*OdenUpdateGetViewportMousePosFunc)(OdenVec2 * pos);
typedef bool (*OdenUpdateIsMouseReleasedSinceLastSwapFunc)(void);
typedef bool (*OdenUpdateDownloadVirtualCameraImage)(
    const char * virtualCameraEntityId, const uint8_t ** packedRgbData, int32_t * width, int32_t * height);
typedef bool (*OdenUpdateVirtualCameraPixelToRay)(
    const char * virtualCameraEntityId, float x, float y, OdenRay * rayOut);

typedef struct OdenMonitorInfo_s {
    OdenVec2 lower_left_corner_px;
    OdenVec2 size_px;
} OdenMonitorInfo;

typedef bool (*OdenUpdateGetMonitorInfo)(OdenMonitorInfo * monitorInfoArrayOut, int * monitorCountInOut);

typedef struct OdenViewportInfo_s {
    int32_t width_px;
    int32_t height_px;
    int32_t origin_x_px;
    int32_t origin_y_px;
    OdenVec2 content_scale;
} OdenViewportInfo;

typedef OdenViewportInfo (*OdenUpdateGetViewportInfoFunc)(void);
typedef OdenVec2 (*OdenUpdateGetViewMousePosPxWithOffsetFunc)(void);

typedef enum OdenPluginImDrawTextAlignment_e {
    OdenPluginImDrawTextAlignmentCentered = 0,
    OdenPluginImDrawTextAlignmentLeft = 1,
    OdenPluginImDrawTextAlignmentRight = 2,
    OdenPluginImDrawTextAlignmentMaxEnum = 0x7FFFFFFF,
} OdenPluginImDrawTextAlignment;

typedef void (*OdenUpdateImDrawSetUseZBufferFunc)(bool enable);
typedef void (*OdenUpdateImDrawSetWriteZBufferFunc)(bool enable);
typedef void (*OdenUpdateImDrawPushMatrixFunc)(OdenMatrix4 mat);
typedef void (*OdenUpdateImDrawPopMatrixFunc)(void);
typedef void (*OdenUpdateImDrawPushViewSpace)(void);
typedef void (*OdenUpdateImDrawPopViewSpace)(void);
typedef void (*OdenUpdateImDrawSetViewSpaceZ)(int32_t z);
typedef void (*OdenUpdateImDrawSetTint)(OdenVec4 tint);
typedef void (*OdenUpdateImDrawPushShaderFunc)(int32_t shaderId);
typedef void (*OdenUpdateImDrawPopShaderFunc)(void);
typedef void (*OdenUpdateImDrawLatchBackbufferFunc)(int32_t blurPasses);

typedef void (*OdenUpdateImDrawAddLineFunc)(
    OdenVec3 from, OdenVec3 to, OdenVec3 normal, float width, OdenVec4 color);
typedef void (*OdenUpdateImDrawAddCircleSegmentFunc)(
    OdenVec3 pos,
    float radius,
    float fromAngleRad,
    float toAngleRad,
    float width,
    OdenVec3 normal,
    OdenVec4 color,
    int32_t circleSegments);
typedef void (*OdenUpdateImDrawAddCircleFunc)(
    OdenVec3 pos, float radius, float width, OdenVec3 normal, OdenVec4 color, int32_t circleSegments);
typedef void (*OdenUpdateImDrawAddSquareFunc)(OdenVec3 pos, OdenVec2 size, float width, OdenVec4 color);
typedef void (*OdenUpdateImDrawAddRectangleFunc)(OdenVec3 pos, OdenVec2 size, OdenVec4 color);
typedef void (*OdenUpdateImDrawAddRectangleRoundedFunc)(
    OdenVec3 pos, OdenVec2 size, float rounding, OdenVec4 color);
typedef void (*OdenUpdateImDrawAddImageFunc)(OdenVec3 pos, OdenVec2 size, int32_t imageId);
typedef void (*OdenUpdateImDrawAddTextFunc)(
    const char * text, OdenVec3 pos, OdenVec3 up, OdenVec3 right, OdenVec4 color, int32_t fontId);
typedef void (*OdenUpdateImDrawAddTextNormalFunc)(
    const char * text, OdenVec3 pos, OdenVec3 normal, float size, OdenVec4 color, int32_t fontId);
typedef void (*OdenUpdateImDrawAddTextAlignedFunc)(
    const char * text,
    OdenVec3 pos,
    OdenVec2 boundingBox,
    float textHeight,
    OdenVec3 up,
    OdenVec3 right,
    OdenPluginImDrawTextAlignment alignment,
    OdenVec4 color,
    int32_t fontId);
typedef void (*OdenUpdateImDrawAddTextAlignedNormalFunc)(
    const char * text,
    OdenVec3 pos,
    OdenVec2 boundingBox,
    float textHeight,
    OdenVec3 normal,
    OdenPluginImDrawTextAlignment alignment,
    OdenVec4 color,
    int32_t fontId);
typedef void (*OdenUpdateImDrawAddTorusFunc)(
    OdenVec3 pos, OdenVec3 normal, float circleRadius, float tubeRadius, OdenVec4 color);
typedef void (*OdenUpdateImDrawAddLinesFunc)(
    const OdenVec3 * point, int32_t pointCount, OdenVec3 normal, float width, OdenVec4 color);
typedef void (*OdenUpdateImDrawAddRectangleBorderFunc)(
    OdenVec3 pos, OdenVec2 size, float thickness, OdenVec4 color);
typedef void (*OdenUpdateImDrawAddRectangleBorderRoundedFunc)(
    OdenVec3 pos,
    OdenVec2 size,
    float thickness,
    float innerCornerRaduis,
    int32_t innerCornerSegments,
    OdenVec4 color);
typedef void (*OdenUpdateImDrawAddCuboidFunc)(OdenVec3 pos, OdenVec3 size, OdenVec4 color);
typedef void (*OdenUpdateImDrawAddTrianglesFunc)(
    OdenVec3 * vertexPositions, OdenVec4 * vertexColors, int32_t vertexCount);
typedef void (*OdenUpdateImDrawPushScreenSpace)(void);
typedef void (*OdenUpdateImDrawPopScreenSpace)(void);
typedef void (*OdenUpdateImDrawAddVideoFunc)(
    OdenVec3 pos, OdenVec2 size, const char * entityId, int32_t stream);
typedef void (*OdenUpdateImDrawAddTrianglesWithUvsFunc)(
    const OdenVec3 * vertexPositions,
    const OdenVec2 * vertexUvs,
    const OdenVec4 * vertexColors,
    int32_t vertexCount);
typedef void (*OdenUpdateImDrawAddVideoWithUvsFunc)(
    OdenVec3 pos,
    OdenVec2 size,
    const char * entityId,
    int32_t stream,
    OdenVec2 topLeftUv,
    OdenVec2 bottomRightUv);

typedef void (*OdenUpdateImDrawAddRectangleAlignedFunc)(
    OdenVec3 pos, OdenVec3 up, OdenVec3 right, OdenVec2 size, OdenVec4 color);
typedef void (*OdenUpdateImDrawAddRectangleRoundedAlignedFunc)(
    OdenVec3 pos, OdenVec3 up, OdenVec3 right, OdenVec2 size, float rounding, OdenVec4 color);
typedef void (*OdenUpdateImDrawAddRectangleBorderAlignedFunc)(
    OdenVec3 pos, OdenVec3 up, OdenVec3 right, OdenVec2 size, float thickness, OdenVec4 color);
typedef void (*OdenUpdateImDrawAddRectangleBorderRoundedAlignedFunc)(
    OdenVec3 pos,
    OdenVec3 up,
    OdenVec3 right,
    OdenVec2 size,
    float thickness,
    float innerCornerRadius,
    int32_t innerCornerSegments,
    OdenVec4 color);

typedef bool (*OdenUpdateImDrawIsMouseOverLastElementFunc)(void);
typedef bool (*OdenUpdateImDrawIsMouseClickedLastElementFunc)(void);
typedef bool (*OdenUpdateImDrawIsMousePressedOverLastElementFunc)(void);
typedef void (*OdenUpdateImDrawRemoveLastElementFunc)(void);
typedef OdenVec2 (*OdenUpdateimDrawCalcTextSizeFunc)(
    const char * text, float textHeight, float maxWidth, int32_t fontId);

// Input functions

typedef bool (*OdenInputIsKeyDownFunc)(int32_t key, const int32_t * modifiers);
typedef bool (*OdenInputIsKeyPressedSinceLastSwapFunc)(int32_t key, const int32_t * modifiers);
typedef bool (*OdenInputGetKeysPressedSinceLastSwap)(int32_t * keys, int32_t * modifiers, int32_t * size);
typedef bool (*OdenInputGetWindowEventsFunc)(OdenWindowEvent * eventBuffer, int32_t * size);

// Managed Com Channel functions

typedef int32_t (*OdenComChannelMaxMessageSizeFunc)(void);
typedef void (*OdenComChannelSendMessageFunc)(const char * messageId, const uint8_t * data, int32_t dataSize);
typedef int32_t (*OdenComChannelGetMessageCountFunc)(const char * messageId);
typedef bool (*OdenComChannelGetMessageFunc)(
    const char * messageId, int32_t index, uint8_t * data, int32_t dataSize);
typedef bool (*OdenComChannelGetLastMessageFunc)(const char * messageId, uint8_t * data, int32_t dataSize);

typedef bool (*OdenComChannelGetVarSizeMsgFunc)(
    const char * messageId, int32_t index, uint8_t * data, int32_t * dataSize);
typedef bool (*OdenComChannelGetLastVarSizeMsgFunc)(
    const char * messageId, uint8_t * data, int32_t * dataSize);

// Neural Network functions

typedef OdenNeuralNetworkError (*OdenAddNeuralNetworkFunc)(OdenUuid * uuid);
typedef OdenNeuralNetworkError (*OdenRemoveNeuralNetworkFunc)(const OdenUuid * uuid);
typedef OdenNeuralNetworkError (*OdenSetNeuralNetworkModelFunc)(const OdenUuid * uuid, const char * filePath);
typedef OdenNeuralNetworkError (*OdenSetNeuralNetworkInputVideoStreamFunc)(
    const OdenUuid * uuid, const char * tensorName, const char * entityId, int32_t streamId);
typedef OdenNeuralNetworkError (*OdenGetNeuralNetworkTensorDataFunc)(
    const OdenUuid * uuid, const char * tensorName, OdenTensorData * tensorData);
typedef OdenNeuralNetworkError (*OdenTriggerNeuralNetworkRunFunc)(const OdenUuid * uuid, int32_t * frameId);
typedef OdenNeuralNetworkError (*OdenGetNeuralNetworkInfoFunc)(
    const OdenUuid * uuid, OdenNeuralNetworkInfo * neuralNetworkInfo);
typedef OdenNeuralNetworkError (*OdenGetInferenceSettingsFunc)(OdenInferenceSettings * settings);
typedef OdenNeuralNetworkError (*OdenSetNeuralNetworkDlaCoreFunc)(const OdenUuid * uuid, int32_t dlaCore);

// Scene functions

typedef struct OdenCameraMetadata_s {
    union {
        DEPRECATED("Use availableMetadata") int32_t avalibleMetadata;
        int32_t availableMetadata;
    };
    uint8_t metadata[200];
    uint64_t timestamp;
} OdenCameraMetadata;

typedef struct OdenBasicStatistics_s {
    float ping;
    float channelUsage;
    float packetLoss;
    float kiloBytesInFlight;
    float bandwidthMbit;
    float roundTripTimeMs;
} OdenBasicStatistics;

typedef struct OdenStreamerLinkFeedbackStats_s {
    char networkDeviceName[ODEN_NETWORK_DEVICE_NAME_SIZE + 1];
    float channelUsage;
    float streamerBitrateMbps;
    float estimatedMbpsForLink;
    int64_t feedbackAgeNs;
    bool hasValidFeedback;
} OdenStreamerLinkFeedbackStats;

typedef struct OdenStreamPacketLoss_s {
    int64_t totalPackets;
    int64_t totalLostPackets;
} OdenStreamPacketLoss;

typedef struct OdenStreamStatistics_s {
    int64_t frameEmitTime;
    float channelUsage;
    int32_t lostPackets;
    int32_t frameNo;
    int64_t firstPacketTime;
    int64_t lastPacketTime;
} OdenStreamStatistics;

typedef struct OdenReceiveFrameStatistics_s {
    int64_t frameEmitTime;
    int32_t lostPackets;
    int32_t frameNo;
    float channelUsage;
    int64_t firstPacketTime;
    int64_t lastPacketTime;
    int32_t frameSize;
} OdenReceiveFrameStatistics;

typedef struct OdenReceiveInterfaceStatistics_s {
    uint32_t ip;
    uint16_t port;

    OdenReceiveFrameStatistics frameStatistics[64];
} OdenReceiveInterfaceStatistics;

typedef struct OdenStreamStatisticsEx_s {

    OdenReceiveFrameStatistics frameStatistics[64];

    int32_t interfaceStatisticsCount;
    OdenReceiveInterfaceStatistics interfaceStatistics[64];

} OdenStreamStatisticsEx;

typedef enum OdenCameraStreamState_e {
    OdenCameraStreamStateUnknown = 0,
    OdenCameraStreamStateStopped = 1,
    OdenCameraStreamStatePaused = 2,
    OdenCameraStreamStatePlaying = 3,
    OdenCameraStreamStateMaxEnum = 0x7FFFFFFF,
} OdenCameraStreamState;

typedef enum OdenLinkMode_e {
    OdenLinkModeReceiver = 0,
    OdenLinkModeSender = 1,
    OdenLinkModeMaxEnum = 0x7FFFFFFF,
} OdenLinkMode;

typedef enum OdenLinkError_e {
    OdenLinkErrorOk = 0,
    OdenLinkErrorUnknown = 1,
    OdenLinkErrorInvalidEntityId = 2,
    OdenLinkErrorInvalidLinkIndex = 3,
    OdenLinkErrorNoLinksConfigured = 4,
    OdenLinkErrorPortOutOfRange = 5,
    OdenLinkErrorInvalidIpAddress = 6,
    OdenLinkErrorInsufficientBufferSize = 7,
    OdenLinkErrorArgumentIsNull = 8,
    OdenLinkErrorInvalidLinkMode = 9,
    OdenLinkErrorInvalidPublicKeyIndex = 10,
    OdenLinkErrorInvalidPublicKey = 11,
    OdenLinkErrorInvalidType = 12,
    OdenLinkErrorInvalidLinkUuid = 13,
    OdenLinkErrorNoTrafficOnLink = 14,
    OdenLinkErrorMaxEnum = 0x7FFFFFFF,
} OdenLinkError;

typedef enum OdenLinkP2pStatus_e {
    OdenLinkP2pStatusInactive = 0,
    OdenLinkP2pStatusQueryingStunServer = 1,
    OdenLinkP2pStatusNegotiatingAddresses = 2,
    OdenLinkP2pStatusHolePunching = 3,
    OdenLinkP2pStatusVerifyingConnection = 4,
    OdenLinkP2pStatusConnectionEstablished = 5,
    OdenLinkP2pStatusMaxEnum = 0x7FFFFFFF,
} OdenLinkP2pStatus;

typedef struct OdenLinkP2PStatusWrapper_s {
    OdenLinkP2pStatus status;
    char sourceIp[64];
    char destinationIp[64];
} OdenLinkP2PStatusWrapper;

typedef enum OdenLinkStatsStructureType_e {
    OdenLinkStatsStructureTypeTransferredBytes = 0,
    OdenLinkStatsStructureTypeInterfaceStatistics = 1,
    OdenLinkStatsStructureTypeMaxEnum = 0x7FFFFFFF,
} OdenLinkStatsStructureType;

typedef struct OdenLinkStatsTransferredBytes_s {
    OdenLinkStatsStructureType type;
    void * next;

    uint64_t bytesReceived;
    uint64_t bytesSent;
} OdenLinkStatsTransferredBytes;

typedef struct OdenLinkStatsInterfaceStatistics_s {
    OdenLinkStatsStructureType type;
    void * next;

    OdenReceiveInterfaceStatistics statistics;
    int64_t timeSinceLastReceivedPacketNs;
} OdenLinkStatsInterfaceStatistics;

typedef struct OdenAlignmentId_s {
    int32_t alignmentId;
    OdenUuid remoteStreamerUuid;
} OdenAlignmentId;

typedef struct OdenVideoPlacement_s {
    int32_t left;
    int32_t top;
    int32_t width;
    int32_t height;
    int32_t rotationDeg;
} OdenVideoPlacement;

typedef enum OdenWindowMode_s {
    OdenWindowModeWindowed = 0,
    OdenWindowModeWindowedFullscreen = 1,
    OdenWindowModeSpanningFullscreen = 2,
    OdenWindowModeFullscreen = 3,
    OdenWindowModeMaxEnum = 0x7FFFFFFF,
} OdenWindowMode;

typedef struct OdenFlexboxLayout_s {
    OdenUuid uuid;
    int32_t top;
    int32_t left;
    int32_t width;
    int32_t height;
} OdenFlexboxLayout;

typedef enum OdenColliderType_s {
    OdenColliderTypeBox = 0,
    OdenColliderTypeCylinder = 1,
    OdenColliderTypeMaxEnum = 0x7FFFFFFF,
} OdenColliderType;

typedef struct OdenCollider_s {
    OdenColliderType colliderType;
    OdenVec3 offset;
    OdenQuat rotation;
    OdenVec3 size;
    float density;
    float frictionCoefficient;
} OdenCollider;

typedef struct OdenRigidBody_s {
    char entityId[256];
    OdenCollider colliders[128];
    int32_t colliderCount;
    bool dynamic;
} OdenRigidBody;

typedef struct OdenRigidBodyUuid_s {
    char entityName[256];
    OdenUuid entityUuid;
    OdenCollider colliders[128];
    int32_t colliderCount;
    bool dynamic;
} OdenRigidBodyUuid;

typedef enum OdenSceneParamError_e {
    OdenSceneParamErrorOk = 0,
    OdenSceneParamErrorError = 1,
    OdenSceneParamErrorInvalidType = 2,
    OdenSceneParamErrorInvalidEntity = 3,
    OdenSceneParamErrorInvalidValue = 4,
    OdenSceneParamErrorWrongApplication = 5,
    OdenSceneParamErrorUnsupportedGet = 6,
    OdenSceneParamErrorUnsupportedSet = 7,
    OdenSceneParamErrorArgumentIsNullptr = 8,
    OdenSceneParamErrorTypeAndStructMismatch = 9,
    OdenSceneParamErrorVideoStreamNotAvailable = 10,
    OdenSceneParamErrorUnsupportedForOS = 11,
    OdenSceneParamErrorMaxEnum = 0x7FFFFFFF,
} OdenSceneParamError;

typedef enum OdenSceneParamType_e {
    OdenSceneParamTypeOutputAlignmentPosition = 0,
    OdenSceneParamTypeOutputAlignmentScale = 1,
    OdenSceneParamTypeOutputAlignmentRotation = 2,
    OdenSceneParamTypeStreamerResolution = 3,
    OdenSceneParamTypeSyncCamera = 4,
    OdenSceneParamTypeDecoderStats = 5,
    OdenSceneParamTypeGuiHovered = 6,
    OdenSceneParamTypePrintScreen = 7,
    OdenSceneParamTypeWindowMode = 8,
    OdenSceneParamTypeLinkIndex = 9,
    OdenSceneParamTypeLinkUuid = 10,
    OdenSceneParamTypeBandwidthControlState = 11,
    OdenSceneParamTypeSceneAsString = 12,
    OdenSceneParamTypeLoadSceneFromString = 13,
    OdenSceneParamTypeSceneEntityUuids = 14,
    OdenSceneParamTypeFlexboxLayout = 15,
    OdenSceneParamTypeTargetBitrate = 16,
    OdenSceneParamTypeGetEntityEnabled = 17,
    OdenSceneParamTypeSceneAsStringEx = 18,
    OdenSceneParamTypeOutputAlignmentId = 19,
    OdenSceneParamTypeCloneSourceUuid = 20,
    OdenSceneParamTypeRigidBodies = 21,
    OdenSceneParamTypeGuiVisible = 22,
    OdenSceneParamTypeVirtualCameraResolution = 23,
    OdenSceneParamTypeStitchedVideoCalibrationQuality = 24,
    OdenSceneParamTypeRawRecordingFolder = 25,
    OdenSceneParamTypeVirtualCameraOnlyRenderUuids = 26,
    OdenSceneParamTypeRayToVideoIntersect = 27,
    OdenSceneParamTypeFlexboxRotation = 28,
    OdenSceneParamTypePerformanceMetrics = 29,
    OdenSceneParamTypeLinkEncryption = 30,
    OdenSceneParamTypeRigidBodiesUuid = 31,
    OdenSceneParamTypeRemoteStreamerVideos = 32,
    OdenSceneParamTypeStreamName = 33,
    OdenSceneParamTypePackerEnabled = 34,
    OdenSceneParamTypePackerAutoCrop = 35,
    OdenSceneParamTypePackerTargetRemoteStreamer = 36,
    OdenSceneParamTypeWindowFocus = 37,
    OdenSceneParamTypeAudioOutputVolume = 38,
    OdenSceneParamTypeLinkBindDevice = 39,
    OdenSceneParamTypeInhibitHotkeys = 40,
    OdenSceneParamTypeCameraCrop = 41,
    OdenSceneParamTypeCameraHardCrop = 42,
    OdenSceneParamTypeEntityScale = 43,
    OdenSceneParamTypeBackgroundColor = 44,
    OdenSceneParamTypeClearBackgroundColor = 45,
    OdenSceneParamTypeDropDetectorTimeout = 46,
    OdenSceneParamTypeShowNoSignalScreen = 47,
    OdenSceneParamTypeMaxReorderFrames = 48,
    OdenSceneParamTypeRegulatorMaxPacketSize = 49,
    OdenSceneParamTypeGuiTextInput = 50,
    OdenSceneParamTypeAudioLinkState = 51,
    OdenSceneParamTypeNetworkLog = 52,
    OdenSceneParamTypeMaxEnum = 0x7FFFFFFF,
} OdenSceneParamType;

typedef struct OdenSceneParamOutputAlignmentPosition_s {
    OdenSceneParamType type;
    void * next;

    const char * entityId;
    OdenVec2i position;
} OdenSceneParamOutputAlignmentPosition;

typedef struct OdenSceneParamOutputAlignmentScale_s {
    OdenSceneParamType type;
    void * next;

    const char * entityId;
    float scale;
} OdenSceneParamOutputAlignmentScale;

typedef enum OdenRotation_e {
    OdenRotation0 = 0,
    OdenRotation90 = 1,
    OdenRotation180 = 2,
    OdenRotation270 = 3,
    OdenRotationMaxEnum = 0x7FFFFFFF,
} OdenRotation;

// OdenRotation has got a new name, this lines are to keep backwards compatibility
typedef OdenRotation OdenOutputAlignmentRotation;
#define OdenOutputAlignmentRotation0       OdenRotation0
#define OdenOutputAlignmentRotation90      OdenRotation90
#define OdenOutputAlignmentRotation180     OdenRotation180
#define OdenOutputAlignmentRotation270     OdenRotation270
#define OdenOutputAlignmentRotationMaxEnum OdenRotationMaxEnum

typedef struct OdenFlexboxRotation_s {
    OdenUuid uuid;
    OdenRotation rot;
} OdenFlexboxRotation;

typedef struct OdenSceneParamOutputAlignmentRotation_s {
    OdenSceneParamType type;
    void * next;

    const char * entityId;
    OdenOutputAlignmentRotation rotation;
} OdenSceneParamOutputAlignmentRotation;

typedef struct OdenSceneParamStreamerResolution_s {
    OdenSceneParamType type;
    void * next;

    OdenVec2i resolution;
} OdenSceneParamStreamerResolution;

typedef struct OdenSceneParamSyncCamera_s {
    OdenSceneParamType type;
    void * next;

    const char * entityOrStream;
    int32_t stream;
} OdenSceneParamSyncCamera;

typedef struct OdenDecodedFrame_s {
    int32_t frameNo;
    bool decodeSuccessful;
} OdenDecodedFrame;

typedef struct OdenSceneParamDecoderStats_s {
    OdenSceneParamType type;
    void * next;

    const char * entityId;
    int32_t stream;
    OdenDecodedFrame decodedFrames[64];
} OdenSceneParamDecoderStats;

typedef struct OdenSceneParamGuiHovered_s {
    OdenSceneParamType type;
    void * next;

    bool isHovered;
} OdenSceneParamGuiHovered;

typedef struct OdenSceneParamPrintScreen_s {
    OdenSceneParamType type;
    void * next;

    int32_t width;
    int32_t height;
    uint8_t * data;
    int32_t size;
} OdenSceneParamPrintScreen;

typedef struct OdenSceneParamWindowMode_s {
    OdenSceneParamType type;
    void * next;

    OdenWindowMode mode;
} OdenSceneParamWindowMode;

typedef struct OdenSceneParamLinkIndex_s {
    OdenSceneParamType type;
    void * next;

    const char * entityId;
    const OdenUuid * uuid;
    int32_t index;

    OdenLinkError result;
} OdenSceneParamLinkIndex;

typedef struct OdenSceneParamLinkUuid_s {
    OdenSceneParamType type;
    void * next;

    const char * entityId;
    int32_t index;
    OdenUuid uuid;

    OdenLinkError result;
} OdenSceneParamLinkUuid;

typedef struct OdenSceneParamBandwidthControlState_s {
    OdenSceneParamType type;
    void * next;

    bool enabled;
} OdenSceneParamBandwidthControlState;

typedef struct OdenSceneParamSceneAsString_s {
    OdenSceneParamType type;
    void * next;

    const char * sceneId;
    char * sceneString;
    int32_t * size;
} OdenSceneParamSceneAsString;

typedef struct OdenSceneParamLoadSceneFromString_s {
    OdenSceneParamType type;
    void * next;

    const char * sceneString;
} OdenSceneParamLoadSceneFromString;

typedef struct OdenSceneParamSceneEntityUuids_s {
    OdenSceneParamType type;
    void * next;

    int32_t sceneId;
    OdenUuid * uuids;
    int32_t * uuidCount;
} OdenSceneParamSceneEntityUuids;

typedef struct OdenSceneParamFlexboxLayout_s {
    OdenSceneParamType type;
    void * next;

    const char * entityId;
    OdenVec4 * padding;
    OdenFlexboxLayout * layouts;
    int32_t * layoutCount;
} OdenSceneParamFlexboxLayout;

typedef struct OdenSceneParamTargetBitrate_s {
    OdenSceneParamType type;
    void * next;

    float bitrate;
} OdenSceneParamTargetBitrate;

typedef struct OdenSceneParamGetEntityEnabled_s {
    OdenSceneParamType type;
    void * next;

    const char * entityNameOrUuid;
    bool * enabled;
} OdenSceneParamGetEntityEnabled;

typedef struct OdenSceneParamSceneAsStringEx_s {
    OdenSceneParamType type;
    void * next;

    int32_t sceneId;
    char * sceneString;
    int32_t * size;

    bool fullSceneConfig;
} OdenSceneParamSceneAsStringEx;

typedef struct OdenSceneParamOutputAlignmentId_s {
    OdenSceneParamType type;
    void * next;

    const char * entityId;
    int32_t * alignmentId;
} OdenSceneParamOutputAlignmentId;

typedef struct OdenSceneParamCloneSourceUuid_s {
    OdenSceneParamType type;
    void * next;

    const char * entityId;
    int32_t stream;
    OdenUuid * uuid;

} OdenSceneParamCloneSourceUuid;

typedef struct OdenSceneParamRigidBodies_s {
    OdenSceneParamType type;
    void * next;

    OdenRigidBody * rigidBodies;
    int32_t * rigidBodyCount;
} OdenSceneParamRigidBodies;

typedef struct OdenSceneParamRigidBodiesUuid_s {
    OdenSceneParamType type;
    void * next;

    OdenRigidBodyUuid * rigidBodies;
    int32_t * rigidBodyCount;
} OdenSceneParamRigidBodiesUuid;

typedef struct OdenSceneParamGuiVisible_s {
    OdenSceneParamType type;
    void * next;

    bool visible;
} OdenSceneParamGuiVisible;

typedef struct OdenSceneParamVirtualCameraResolution_s {
    OdenSceneParamType type;
    void * next;

    const char * entityId;
    OdenVec2i resolution;
} OdenSceneParamVirtualCameraResolution;

typedef struct OdenSceneParamStitchedVideoQuality_s {
    OdenSceneParamType type;
    void * next;

    const char * entityId;
    float quality;
    float confidence;
} OdenSceneParamStitchedVideoQuality;

typedef struct OdenSceneParamRawRecordingFolder_s {
    OdenSceneParamType type;
    void * next;

    char * path;
    int32_t * pathSize;
} OdenSceneParamRawRecordingFolder;

typedef struct OdenSceneParamPackerEnabled_s {
    OdenSceneParamType type;
    void * next;

    bool enabled;
} OdenSceneParamPackerEnabled;

typedef struct OdenSceneParamPackerAutoCrop_s {
    OdenSceneParamType type;
    void * next;

    bool enabled;
} OdenSceneParamPackerAutoCrop;

typedef struct OdenSceneParamPackerTargetRemoteStreamer_s {
    OdenSceneParamType type;
    void * next;

    const char * remoteStreamerId;
    OdenUuid * targetRemoteStreamerUuid;
} OdenSceneParamPackerTargetRemoteStreamer;

typedef struct OdenSceneParamVirtualCameraOnlyRenderUuids_s {
    OdenSceneParamType type;
    void * next;

    const char * entityId;
    OdenUuid * uuids;
    int32_t * uuidCount;
} OdenSceneParamVirtualCameraOnlyRenderUuids;

typedef struct OdenSceneParamRayToVideoIntersect_s {
    OdenSceneParamType type;
    void * next;

    const char * entityId;
    OdenRay ray;
    int32_t * resultCount;
    OdenRayToVideoIntersect * intersectResult;
} OdenSceneParamRayToVideoIntersect;

typedef struct OdenSceneParamFlexboxRotation_s {
    OdenSceneParamType type;
    void * next;

    const char * entityId;
    OdenFlexboxRotation * rotations;
    int32_t * rotationsCount;
} OdenSceneParamFlexboxRotation;

typedef struct OdenSceneParamLinkEncryption_s {
    OdenSceneParamType type;
    void * next;

    const char * entityId;
    int32_t index;
    bool enable;

    OdenLinkError result;
} OdenSceneParamLinkEncryption;

typedef struct OdenInferenceMetrics_s {
    const char * networkPath;
    OdenUuid uuid;
    double inferenceTimeS;

    uint8_t reserved[8];
} OdenInferenceMetrics;

typedef struct OdenSceneParamPerformanceMetrics_s {
    OdenSceneParamType type;
    void * next;

    // CPU Time
    double totalFrameTimeS;
    double updateTimeS;
    double drawTimeS;
    double guiTimeS;

    // GPU Time
    double gpuDrawTimeS;

    // Streamer specific
    double encodeTimeS;

    // Inference
    OdenInferenceMetrics inferenceMetrics[8];
    uint8_t inferenceMetricsCount;

    uint8_t reserved[16];
} OdenSceneParamPerformanceMetrics;

typedef struct OdenRemoteStreamerVideo_s {
    int32_t stream;
    int32_t width;
    int32_t height;
} OdenRemoteStreamerVideo;

typedef struct OdenSceneParamRemoteStreamerVideos_s {
    OdenSceneParamType type;
    void * next;

    const char * remoteStreamerEntity;
    OdenRemoteStreamerVideo * videos;
    int32_t * videosCount;
} OdenSceneParamRemoteStreamerVideos;

typedef struct OdenSceneParamStreamName_s {
    OdenSceneParamType type;
    void * next;

    const char * entityId;
    int32_t stream;
    char * streamName;
    int32_t * streamNameLen;
} OdenSceneParamStreamName;

typedef struct OdenSceneParamWindowFocus_s {
    OdenSceneParamType type;
    void * next;

    bool focused;
} OdenSceneParamWindowFocus;

typedef struct OdenSceneParamAudioOutputVolume_s {
    OdenSceneParamType type;
    void * next;

    const char * remoteStreamerId;
    float volume;
} OdenSceneParamAudioOutputVolume;

typedef struct OdenSceneParamLinkBindDevice_s {
    OdenSceneParamType type;
    void * next;

    const char * entityId;
    int32_t index;

    char * buffer;
    int32_t * buffer_lenght;

    OdenLinkError result;
} OdenSceneParamLinkBindDevice;

typedef struct OdenSceneParamInhibitHotkeys_s {
    OdenSceneParamType type;
    void * next;
} OdenSceneParamInhibitHotkeys;

typedef struct OdenSceneParamCameraCrop_s {
    OdenSceneParamType type;
    void * next;

    const char * entityId;
    int32_t streamIndex;
    OdenCameraCropData cropData;
} OdenSceneParamCameraCrop;

typedef struct OdenSceneParamCameraHardCrop_s {
    OdenSceneParamType type;
    void * next;

    const char * entityId;
    int32_t streamIndex;
    OdenVec4 hardCropData;
} OdenSceneParamCameraHardCrop;

typedef struct OdenSceneParamEntityScale_s {
    OdenSceneParamType type;
    void * next;

    const char * entityId;
    OdenVec3 scale;
} OdenSceneParamEntityScale;

typedef struct OdenSceneParamBackgroundColor_s {
    OdenSceneParamType type;
    void * next;

    OdenVec3 color;
} OdenSceneParamBackgroundColor;

typedef struct OdenSceneParamClearBackgroundColor_s {
    OdenSceneParamType type;
    void * next;
} OdenSceneParamClearBackgroundColor;

typedef struct OdenSceneParamDropDetectorTimeout_s {
    OdenSceneParamType type;
    void * next;

    const char * entityId;
    int32_t streamIndex;
    float timeoutMs;
} OdenSceneParamDropDetectorTimeout;

typedef struct OdenSceneParamShowNoSignalScreen_s {
    OdenSceneParamType type;
    void * next;

    const char * entityId;
    int32_t streamIndex;
    bool show;
} OdenSceneParamShowNoSignalScreen;

typedef struct OdenSceneParamMaxReorderFrames_s {
    OdenSceneParamType type;
    void * next;

    const char * entityId;
    int32_t streamIndex;
    int32_t maxReorderFrames;
} OdenSceneParamMaxReorderFrames;

typedef struct OdenSceneParamRegulatorMaxPacketSize_s {
    OdenSceneParamType type;
    void * next;

    OdenRegulatorMode mode;
    int32_t maxPacketSize;
} OdenSceneParamRegulatorMaxPacketSize;

typedef struct OdenSceneParamGuiTextInput_s {
    OdenSceneParamType type;
    void * next;

    bool isActive;
} OdenSceneParamGuiTextInput;

typedef struct OdenSceneParamAudioLinkState_s {
    OdenSceneParamType type;
    void * next;

    const char * remoteStreamerId;
    // True once the audio device has opened
    bool receiving;
    bool sending;
    // True from the start request until stop, also while the device is still opening
    bool receiveRequested;
    bool sendRequested;
} OdenSceneParamAudioLinkState;

typedef struct OdenSceneParamNetworkLog_s {
    OdenSceneParamType type;
    void * next;

    bool enable;
    int32_t maxSizeMb;
    int32_t rotateMinutes;
    const char * directory;
} OdenSceneParamNetworkLog;

typedef bool (*OdenGetHorizonFunc)(const char * entity, OdenVec3 * horizon);
typedef bool (*OdenSetHorizonFunc)(const char * entity, OdenVec3 horizon);
typedef bool (*OdenSetCurtainFunc)(
    const char * entity, float curtain_global, float curtain_lower, float curtain_upper);
typedef void (*OdenSetSeparateProjectionsFunc)(bool enabled);
typedef bool (*OdenSetStitchDistanceFunc)(const char * entity, float dist);
typedef void (*OdenSetCameraBrightnessFunc)(const char * entity, int32_t stream, int32_t brightness);
typedef void (*OdenGetAllConnectedGigECameraSerialsFunc)(
    uint64_t * cameraSerials, int32_t * cameraSerialsCount);
typedef bool (*OdenSetPanoramaVideoSettingsStringFunc)(const char * entity, const char * pv_settings);
typedef bool (*OdenSetCameraCalibrationStringFunc)(const char * entity, const char * calibration);
typedef bool (*OdenGetCameraCalibrationFunc)(
    const char * entity, int32_t stream, OdenCameraCalibration * calibration);
typedef bool (*OdenSetCameraCalibrationFunc)(
    const char * entity, int32_t stream, const OdenCameraCalibration * calibration);
typedef bool (*OdenGetDewarpingParamsFunc)(
    const char * entity,
    bool * enabled,
    float * fov_deg,
    float * k,
    float * offset,
    float * rotation_euler_deg);
typedef bool (*OdenSetDewarpingParamsFunc)(
    const char * entity,
    bool enabled,
    const float * fov_deg,
    const float * k,
    const float * offset,
    const float * rotation_euler_deg);
typedef bool (*OdenGetCameraCropDataFunc)(const char * entity, int32_t stream, OdenCameraCropData * cropData);
typedef bool (*OdenGetCameraMetadataFunc)(const char * entity, int32_t stream, OdenCameraMetadata * metadata);
typedef bool (*OdenGetCameraTemperatureFunc)(const char * entity, int32_t stream, float * temperature);
typedef bool (*OdenGetCameraFpsFunc)(const char * entity, int32_t stream, float * fps);
typedef bool (*OdenGetCameraLastValidTimestampFunc)(
    const char * entity, int32_t stream, uint64_t * timestamp);
typedef void (*OdenSetEntityEnabledFunc)(const char * entity, bool enabled);
typedef bool (*OdenGetEntityNameFunc)(const char * entityId, char * entityName, int32_t * nameSize);
typedef void (*OdenSetEntityTransformFunc)(const char * entity, OdenMatrix4 matrix);
typedef void (*OdenSetVirtualCameraPositionFunc)(const char * entity, OdenVec3 pos);
typedef void (*OdenSetVirtualCameraRotationFunc)(const char * entity, OdenVec3 rot);
typedef void (*OdenSetVirtualCameraFovFunc)(const char * entity, float fovDegrees);
typedef void (*OdenSetVirtualCameraResolutionFunc)(const char * entity, int32_t width, int32_t height);
typedef void (*OdenSetVirtualCameraCircularCropFunc)(const char * entity, float crop);
typedef void (*OdenGetFaultsFunc)(int32_t * faultCount, uint32_t * faults);
typedef void (*OdenShowHmdManualAdjustmentPatternFunc)(bool show);
typedef void (*OdenInhibitMouseLookFunc)(void);
typedef bool (*OdenHasEntityWithNameFunc)(const char * entity);
typedef bool (*OdenGetEntityWorldMatrixFunc)(const char * entity, OdenMatrix4 * worldMatrix);
typedef bool (*OdenGetPanoramaVideoSettingsStringFunc)(
    const char * entity, char * pvSettings, int32_t * pvSettingsCount);
typedef bool (*OdenGetCameraRigSettingsStringFunc)(
    const char * entity, char * cameraRigSettings, int32_t * cameraRigSettingsCount);
typedef bool (*OdenGetCameraCurrentTimestampFunc)(const char * entity, int32_t stream, uint64_t * timestamp);
typedef bool (*OdenGetStreamSenderIpFunc)(const char * entity, int32_t stream, char * ip, int32_t ipSize);
typedef bool (*OdenGetStreamStatisticsFunc)(
    const char * entity, int32_t stream, OdenStreamStatistics * stats, int32_t * statsCount);
typedef bool (*OdenGetAllSceneIdsFunc)(int32_t * sceneIds, int32_t * sceneIdsCount);
typedef int32_t (*OdenGetSelectedSceneIdFunc)(void);
typedef bool (*OdenSetSelectedSceneFunc)(int32_t sceneId);
typedef bool (*OdenGetSceneNameFunc)(int32_t sceneId, char * sceneName, int32_t * sceneNameCount);
typedef void (*OdenRawRecordStartFunc)(void);
typedef void (*OdenRawRecordStopFunc)(void);
typedef bool (*OdenRawRecordSetEntityRingBufferSizeFunc)(const char * entity, int32_t ringBufferSizeInMB);
typedef bool (*OdenRawRecordEntityRingBufferEnableFunc)(const char * entity);
typedef bool (*OdenPlaybackIsPlayingFunc)(void);
typedef void (*OdenPlaybackStartFunc)(void);
typedef void (*OdenPlaybackPauseFunc)(void);
typedef bool (*OdenPlaybackGetTimesFunc)(
    double * minTimestampSeconds, double * maxTimestampSeconds, double * currentPlaybackTimeSeconds);
typedef void (*OdenPlaybackSetTimeFunc)(double timeSeconds);
typedef void (*OdenPlaybackAdvanceTimeFunc)(double advanceSeconds);
typedef void (*OdenPlaybackSetLoopFunc)(bool loop);
typedef bool (*OdenGetStreamerDestinationIpFunc)(char * ip, int32_t * ipCount);
typedef bool (*OdenGetGigECameraSerial)(const char * entity, int32_t stream, uint64_t * serial);
typedef bool (*OdenSetStreamerFrameMetadata)(const char * data, int32_t size);
typedef bool (*OdenSetCameraResolutionFunc)(
    const char * entity, int32_t stream, int32_t width, int32_t height);
typedef bool (*OdenSetCameraGammaFunc)(const char * entity, int32_t stream, float gamma);
typedef bool (*OdenSetCameraAutoExposureRoiFunc)(
    const char * entity, int32_t stream, const char * serializedCommand);
typedef void (*OdenLoadProjectFunc)(const char * path);
typedef void (*OdenSaveProjectFunc)(void);
typedef bool (*OdenProjectHasUnsavedChangesFunc)(void);
typedef bool (*OdenModelEntitySetPathFunc)(const char * entity, const char * modelPath);
typedef bool (*OdenSetStreamerTargetChannelUtilizationFunc)(float targetChannelUtilization);
typedef bool (*OdenSetStreamerMaxBandwidthFunc)(float maxBandwidthMbit);
typedef bool (*OdenSetCameraStreamStateFunc)(
    const char * entity, int32_t stream, OdenCameraStreamState state);
typedef bool (*OdenGetCameraStreamStateFunc)(
    const char * entity, int32_t stream, OdenCameraStreamState * stateOut);
typedef void (*OdenSetForceFeedbackAngleFunc)(float angle);
typedef bool (*OdenGetStreamStatisticsExFunc)(
    const char * entity, int32_t stream, OdenStreamStatisticsEx * stats);
typedef bool (*OdenGetStreamerBitrateMbpsFunc)(float * bitrateMbpsOut);
typedef bool (*OdenGetStreamerLinkFeedbackStatsFunc)(
    int32_t linkIndex, OdenStreamerLinkFeedbackStats * statsOut);
typedef bool (*OdenGetStreamPacketLossFunc)(
    const char * entity, int32_t stream, OdenStreamPacketLoss * packetLossOut);
typedef void (*OdenStartStreamerFunc)(void);
typedef void (*OdenStopStreamerFunc)(void);
typedef OdenTextureStreamingError (*OdenConfigureTextureStreamingFunc)(
    const OdenTextureStreamingConfig * config);
typedef bool (*OdenIsStreamerRunningFunc)(void);
typedef OdenTextureStreamingError (*OdenStartTextureStreamingFunc)(const char * entity_id, int32_t stream);
typedef OdenTextureStreamingError (*OdenStopTextureStreamingFunc)(const char * entity_id, int32_t stream);
typedef bool (*OdenSetStreamerDestinationIpFunc)(const char * ipAddress);
typedef bool (*OdenSetStreamerDestinationPortFunc)(int32_t portNr);
typedef int32_t (*OdenGetStreamerDestinationPortFunc)(void);
typedef bool (*OdenSetStreamerAudioDestinationPortFunc)(const int32_t portNr);
typedef int32_t (*OdenGetStreamerAudioDestinationPortFunc)(void);
typedef bool (*OdenSetStreamerFeedbackDestinationPortFunc)(const int32_t portNr);
typedef int32_t (*OdenGetStreamerFeedbackDestinationPortFunc)(void);
typedef bool (*OdenSetComChannelDestinationPortFunc)(const char * name, const int32_t portNr);
typedef bool (*OdenGetComChannelDestinationPortFunc)(const char * name, int32_t * portNr);
typedef bool (*OdenSetComChannelReceivePortFunc)(const char * name, const int32_t portNr);
typedef bool (*OdenGetComChannelReceivePortFunc)(const char * name, int32_t * portNr);
typedef bool (*OdenSetVideoReceivePortFunc)(
    const char * entityId, const int32_t portNr, const int32_t inputId);
typedef bool (*OdenGetVideoReceivePortFunc)(const char * entityId, int32_t * portNr, const int32_t inputId);
typedef bool (*OdenSetRealtimeAudioLinkReceivePortFunc)(const char * entityId, const int32_t portNr);
typedef bool (*OdenGetRealtimeAudioLinkReceivePortFunc)(const char * entityId, int32_t * portNr);
typedef bool (*OdenSetRemoteStreamerFeedbackReceivePortFunc)(const char * entityId, const int32_t portNr);
typedef bool (*OdenGetRemoteStreamerFeedbackReceivePortFunc)(const char * entityId, int32_t * portNr);
typedef bool (*OdenRawRecordIsRunningFunc)(void);
typedef bool (*OdenRawRecordGetLastPath)(char * path, int32_t * pathSize);
typedef void (*OdenStartEncodedProducerOutputsFunc)(void);
typedef void (*OdenApplyProjectTemplateFunc)(const char * path);
typedef OdenLinkError (*OdenGetLinkCountFunc)(const char * entityId, int32_t * linkCount);
typedef OdenLinkError (*OdenSetLinkModeFunc)(const char * entityId, int32_t index, OdenLinkMode mode);
typedef OdenLinkError (*OdenGetLinkModeFunc)(const char * entityId, int32_t index, OdenLinkMode * mode);
typedef OdenLinkError (*OdenSetLinkBindIpFunc)(const char * entityId, int32_t index, const char * ip);
typedef OdenLinkError (*OdenGetLinkBindIpFunc)(
    const char * entityId, int32_t index, char * ip, int32_t * ipLen);
typedef OdenLinkError (*OdenGetLinkReceivePortFunc)(const char * entityId, int32_t index, int32_t * port);
typedef OdenLinkError (*OdenSetLinkReceivePortFunc)(const char * entityId, int32_t index, int32_t port);
typedef OdenLinkError (*OdenSetLinkDestinationIpFunc)(const char * entityId, int32_t index, const char * ip);
typedef OdenLinkError (*OdenGetLinkDestinationIpFunc)(
    const char * entityId, int32_t index, char * ip, int32_t * ipLen);
typedef OdenLinkError (*OdenSetLinkDestinationPortFunc)(const char * entityId, int32_t index, int32_t port);
typedef OdenLinkError (*OdenGetLinkDestinationPortFunc)(const char * entityId, int32_t index, int32_t * port);
typedef OdenLinkError (*OdenClearLinkEncryptionAllowedPublicKeysFunc)(const char * entityId, int32_t index);
typedef OdenLinkError (*OdenSetLinkToRelayLinkFunc)(const char * entityId, int32_t index, bool enable);
typedef OdenLinkError (*OdenIsLinkRelayLinkFunc)(const char * entityId, int32_t index, bool * enable);
typedef OdenLinkError (*OdenSetLinkSendToRelayFunc)(const char * entityId, int32_t index, bool enable);
typedef OdenLinkError (*OdenIsLinkSendingToRelayFunc)(const char * entityId, int32_t index, bool * enable);
typedef OdenLinkError (*OdenSetLinkDropOutgoingPackets)(const char * entityId, int32_t index, bool drop);
typedef OdenLinkError (*OdenSetLinkAllowOutgoingFeedbackFunc)(
    const char * entityId, int32_t index, bool allow);
typedef OdenLinkError (*OdenGetLinkTimeSinceLastReceivedPacketNsFunc)(
    const char * entityId, int32_t index, int64_t * timeSinceLastReceivedPacketNs);
typedef bool (*OdenGetAllEntityUuidsFunc)(OdenUuid * uuids, int32_t * uuidCount);
typedef const char * (*OdenGetEntityTypeNameFunc)(const char * entityId);
typedef bool (*OdenGetBasicStatisticsFunc)(const char * entityId, OdenBasicStatistics * statistics);
typedef OdenLinkError (*OdenSetLinkEncryptionPrivateKeyPathFunc)(
    const char * entityId, int32_t index, const char * keyPath);
typedef OdenLinkError (*OdenGetLinkEncryptionPrivateKeyPathFunc)(
    const char * entityId, int32_t index, char * keyPath, int32_t * keyPathLen);
typedef OdenLinkError (*OdenAddLinkEncryptionAllowedPublicKeyFunc)(
    const char * entityId, int32_t index, const char * key);
typedef OdenLinkError (*OdenRemoveLinkEncryptionAllowedPublicKeyFunc)(
    const char * entityId, int32_t index, const char * key);
typedef OdenLinkError (*OdenGetLinkEncryptionAllowedPublicKeysCountFunc)(
    const char * entityId, int32_t index, int32_t * publicKeyCount);
typedef OdenLinkError (*OdenGetLinkEncryptionAllowedPublicKeyFunc)(
    const char * entityId, int32_t linkIndex, int32_t keyIndex, char * key, int32_t * keyLen);
typedef bool (*OdenGetLiveSceneIdFunc)(int32_t * sceneIdOut);
typedef bool (*OdenSetLiveSceneFunc)(int32_t sceneId);
typedef void (*OdenShowHmdControllersFunc)(bool show);
typedef bool (*OdenSetFocusRegionCenterPixelPositionFunc)(const char * entityId, int32_t x, int32_t y);
typedef bool (*OdenGetEntityVideoSizeFunc)(const char * entityId, int32_t stream, OdenVec2i * size);
typedef void (*OdenSetRegulatorModeFunc)(OdenRegulatorMode mode);
typedef OdenRegulatorMode (*OdenGetRegulatorModeFunc)(void);
typedef void (*OdenSetForceFeedbackForceFunc)(float force);
typedef bool (*OdenSetCloneStreamSourceFunc)(
    const char * cloneEntityId, int32_t cloneStream, const char * srcEntityId, int32_t srcStream);
typedef bool (*OdenGetSceneContainingPluginEntityFunc)(int32_t * sceneId);
typedef OdenLinkError (*OdenSetLinkEncryptionInternalSourceIpFunc)(
    const char * entityId, int32_t index, const char * ip);
typedef OdenLinkError (*OdenGetLinkEncryptionInternalSourceIpFunc)(
    const char * entityId, int32_t index, char * ip, int32_t * ipLen);
typedef OdenLinkError (*OdenSetLinkEncryptionInternalDestinationIpFunc)(
    const char * entityId, int32_t index, const char * ip);
typedef OdenLinkError (*OdenGetLinkEncryptionInternalDestinationIpFunc)(
    const char * entityId, int32_t index, char * ip, int32_t * ipLen);
typedef OdenLinkError (*OdenAddLinkFunc)(const char * entityId);
typedef OdenLinkError (*OdenRemoveLinkFunc)(const char * entityId, int32_t index);
typedef bool (*OdenGetInstanceUuidFunc)(OdenUuid * uuid);
typedef OdenLinkError (*OdenSetLinkP2pEnabledFunc)(const char * entityId, int32_t index, bool enable);
typedef OdenLinkError (*OdenGetLinkP2pEnabledFunc)(const char * entityId, int32_t index, bool * enable);
typedef OdenLinkError (*OdenGetLinkP2pStatusFunc)(
    const char * entityId, int32_t index, OdenLinkP2PStatusWrapper * status);
typedef OdenLinkError (*OdenSetLinkP2pStunServerFunc)(
    const char * entityId, int32_t index, const char * addr);
typedef OdenLinkError (*OdenGetLinkP2pStunServerFunc)(
    const char * entityId, int32_t index, char * addr, int32_t * addrLen);
typedef bool (*OdenGetCalibratedFloorHeightFunc)(char const * entityId, float * distance);
typedef bool (*OdenGetCameraLastReceivedFrameTimeFunc)(
    char const * entityId, int32_t stream, uint64_t * timestamp);
typedef bool (*OdenGetAlignmentIdFunc)(
    const char * cloneEntityId, int32_t cloneStream, OdenAlignmentId * alignmentId);
typedef bool (*OdenGetVideoPlacementFunc)(
    const OdenAlignmentId * alignmentId, OdenVideoPlacement * videoPlacement);
typedef const char * (*OdenGetVirtualWebcamNameFunc)(const char * remoteStreamerEntityId);
typedef bool (*OdenUpdateSetAudioReceivingEnabled)(const char * entityId, bool enable);
typedef bool (*OdenUpdateSetAudioSendingEnabled)(const char * entityId, bool enable);
typedef bool (*OdenLoadSceneFunc)(const char * path, OdenVec2i * sceneIdInclusiveRangeOut);
typedef void (*OdenRemoveSceneFunc)(int32_t sceneId);
typedef OdenLinkError (*OdenGetLinkStatisticsFunc)(
    const char * remoteStreamerEntity, int32_t index, void * stats);
typedef OdenSceneParamError (*OdenGetSceneParamFunc)(void * param);
typedef OdenSceneParamError (*OdenSetSceneParamFunc)(const void * param);
typedef void (*OdenCloseProjectFunc)(void);
typedef OdenUuid (*OdenAddEntityFunc)(const char * entity, const char * parent);
typedef void (*OdenRemoveEntityFunc)(const char * entity);
typedef bool (*OdenPollRemoteCalibrationFunc)(
    OdenUuid remoteStreamer,
    OdenUuid remoteEntity,
    int32_t * calibrationCount,
    OdenCameraCalibration * calibrations,
    float * floorHeight);
typedef bool (*OdenSetExternalStitchGeometryFunc)(
    const char * entityId, const float * vertices, int32_t verticesCount);
typedef bool (*OdenGetParentUuidFunc)(const char * entity_id, OdenUuid * parent_uuid);
typedef bool (*OdenGetEntityUuidFunc)(const char * entity_id, OdenUuid * entity_uuid);
typedef void (*OdenSetMouseCursorFunc)(OdenPluginMouseCursor cursor);
typedef OdenLinkError (*OdenAddLinkP2pExcludedSubnetFunc)(
    const char * entityId, int32_t index, const char * subnet);
typedef OdenLinkError (*OdenClearLinkP2pExcludedSubnetsFunc)(const char * entityId, int32_t index);
typedef int32_t (*OdenGetLicenseHashFunc)(char * buffer, int32_t bufferLen);

typedef enum OdenQueryDataResult_e {
    OdenQueryDataResultOk = 0,
    OdenQueryDataResultDataIdNotFound = 1,
    OdenQueryDataResultDataIsNotNotExpectedSize = 2,
    OdenQueryDataResultInvalidParameters = 3,
    OdenQueryDataResultMaxEnum = 0x7FFFFFFF,
} OdenQueryDataResult;

typedef struct OdenQueryDataMetadata_s {
    int64_t timestampPublished;
    int32_t framesSincePublished;
} OdenQueryDataMetadata;

typedef void (*OdenPublishData)(const char * dataId, const void * data, int32_t dataSize);
typedef OdenQueryDataResult (*OdenQueryData)(
    const char * dataId,
    OdenQueryDataMetadata * metadata,
    void ** data, // The supplied data pointer will be valid for the length of the plugin callback
    int32_t expectedDataSize);

typedef void (*OdenSetWindowTitleFunc)(const char * overrideWindowTitle);
typedef const char * (*OdenGetOdenVersionFunc)(void);

// Camera functions

typedef struct OdenCameraState_s {
    OdenVec3 position;
    float yaw;   // radians
    float pitch; // radians
    float fov;   // degrees
} OdenCameraState;

typedef enum OdenFpsCameraType_e {
    OdenFpsCameraTypeFps = 0,
    OdenFpsCameraTypeOrbit = 1,
    denFpsCameraTypeMaxEnum = 0x7FFFFFFF,
} OdenFpsCameraType;

typedef void (*OdenSetFpsCameraFieldOfViewFunc)(float fov /* degrees */);
typedef void (*OdenSetHmdCameraFieldOfViewFactorFunc)(float fov);
typedef void (*OdenGetSavedCameraStatesFunc)(OdenCameraState * cameraStates, int32_t * cameraStatesCount);
typedef void (*OdenGetSavedCameraStateFunc)(int32_t index, OdenCameraState * cameraState);
typedef void (*OdenSetSavedCameraStateFunc)(int32_t index, OdenCameraState cameraState);
typedef void (*OdenLerpToSavedCameraStateFunc)(OdenCameraState cameraState, float time);
typedef bool (*OdenGetFpsCameraStateFunc)(OdenCameraState * cameraState);
typedef bool (*OdenGetFpsCameraTypeFunc)(OdenFpsCameraType * cameraType);
typedef void (*OdenSetFpsCameraTypeFunc)(OdenFpsCameraType);
typedef void (*OdenSetGuiVisibleFunc)(bool guiVisible);
typedef void (*OdenPushHmdPostTransformFunc)(OdenMatrix4 mat);
typedef bool (*OdenGetHmdTransformFunc)(OdenMatrix4 * hmdTransformMatrixOut, bool includePostTransform);

// Image callback registration

typedef struct OdenPluginImageCallbackParams_s {
    void * callbackUserData;
    OdenPluginVideoFrame frame;
} OdenPluginImageCallbackParams;

typedef void (*OdenPluginImageCallbackFunc)(OdenPluginImageCallbackParams * params);

typedef void (*OdenPluginRegisterImageCallbackFunc)(
    const char * streamId, OdenPluginImageCallbackFunc callback, void * callbackUserData);

typedef void (*OdenPluginRegisterLidarCallbackFunc)(
    const char * entityId, void * callback, void * callbackUserData);

// Shutdown callback

typedef void (*OdenPluginShutdownCallbackFunc)(void * callbackUserData);

typedef void (*OdenPluginRegisterShutdownCallbackFunc)(
    OdenPluginShutdownCallbackFunc callback, void * callbackUserData);

// Upload image, Font and shaders

typedef struct OdenPluginImDrawImage_s {
    int32_t width;
    int32_t height;
    const uint8_t * data;
    int32_t size;
} OdenPluginImDrawImage;

typedef enum OdenTextureFormat_e {
    OdenTextureFormatRGBA = 0,
    OdenTextureFormatBGRA = 1,
    OdenTextureFormatRGB = 2,
    OdenTextureFormatMaxEnum = 0x7FFFFFFF,
} OdenTextureFormat;

typedef struct OdenUploadImageParams_s {
    OdenTextureFormat format;
    void * reserved[9];
} OdenUploadImageParams;

typedef struct OdenPluginFontRangeEntry_s {
    int32_t start;
    int32_t end;
} OdenPluginFontRangeEntry;

typedef void (*OdenUploadImageFunc)(int32_t id, OdenPluginImDrawImage image);
typedef void (*OdenUploadImageExFunc)(
    int32_t id, OdenPluginImDrawImage image, const OdenUploadImageParams * params);
typedef void (*OdenUploadFontFunc)(
    int32_t id,
    float glyphSize,
    const uint8_t * const * ttfData,
    const int32_t * sizes,
    const OdenPluginFontRangeEntry * const * ranges);
typedef void (*OdenRegisterGlTexIdFunc)(int32_t id, uint32_t texId);
typedef void (*OdenRegisterIOSurfaceIdFunc)(int32_t id, uint32_t ioSurfaceId);
typedef bool (*OdenUploadShaderFunc)(int32_t shaderId, const char * code);

// Graphics functions

typedef void * (*OdenGlLoaderFunc)(const char * name);
typedef OdenGlLoaderFunc (*OdenGetGlLoaderFunc)(void);
typedef bool (*OdenGetCameraGlTexIdFunc)(const char * entityId, int32_t stream, uint32_t * texIdOut);

// GUI functions

typedef void (*OdenGuiLabelFunc)(const char * text, ...);
typedef bool (*OdenGuiButtonFunc)(const char * label);
typedef bool (*OdenGuiInactiveButtonFunc)(const char * label);
typedef bool (*OdenGuiComboFunc)(
    const char * label,
    int32_t * current_item,
    bool (*items_getter)(void * data, int32_t idx, const char ** out_text),
    void * data,
    int32_t items_count);
typedef void (*OdenGuiInactiveComboFunc)(const char * label);
typedef bool (*OdenGuiSliderIntFunc)(
    const char * label, int32_t * v, int32_t v_min, int32_t v_max, const char * display_format);
typedef bool (*OdenGuiSliderFloatFunc)(
    const char * label, float * v, float v_min, float v_max, const char * display_format);
typedef bool (*OdenGuiSliderFloat3Func)(const char * label, float v[3], float v_min, float v_max);
typedef bool (*OdenGuiCheckboxFunc)(const char * label, bool * v);
typedef bool (*OdenGuiTreeNodeFunc)(const char * label);
typedef void (*OdenGuiTreePopFunc)(void);
typedef bool (*OdenGuiDragIntFunc)(
    const char * label,
    int32_t * v,
    float v_speed,
    int32_t min_value,
    int32_t max_value,
    const char * display_format);
typedef bool (*OdenGuiDragInt2Func)(
    const char * label,
    int32_t v[2],
    float v_speed,
    int32_t min_value,
    int32_t max_value,
    const char * display_format);
typedef bool (*OdenGuiDragInt3Func)(
    const char * label,
    int32_t v[3],
    float v_speed,
    int32_t min_value,
    int32_t max_value,
    const char * display_format);
typedef bool (*OdenGuiDragInt4Func)(
    const char * label,
    int32_t v[4],
    float v_speed,
    int32_t min_value,
    int32_t max_value,
    const char * display_format);
typedef bool (*OdenGuiDragInt5Func)(
    const char * label,
    int32_t v[5],
    float v_speed,
    int32_t min_value,
    int32_t max_value,
    const char * display_format);
typedef bool (*OdenGuiDragFloatFunc)(
    const char * label,
    float * v,
    float v_speed,
    float min_value,
    float max_value,
    const char * display_format,
    float power);
typedef bool (*OdenGuiDragFloat2Func)(
    const char * label,
    float v[2],
    float v_speed,
    float min_value,
    float max_value,
    const char * display_format);
typedef bool (*OdenGuiDragFloat3Func)(
    const char * label,
    float v[3],
    float v_speed,
    float min_value,
    float max_value,
    const char * display_format);
typedef bool (*OdenGuiDragFloat4Func)(
    const char * label,
    float v[4],
    float v_speed,
    float min_value,
    float max_value,
    const char * display_format);
typedef bool (*OdenGuiDragFloat5Func)(
    const char * label,
    float v[5],
    float v_speed,
    float min_value,
    float max_value,
    const char * display_format);
typedef void (*OdenGuiSameLineFunc)(void);
typedef bool (*OdenGuiInputText)(const char * label, char * buf, uint64_t buf_size);
typedef void (*OdenGuiLabelColoredFunc)(OdenVec4 color, const char * text, ...);
typedef void (*OdenGuiTooltipIfLastItemHoveredFunc)(const char * text, ...);
typedef void (*OdenGuiGraphFunc)(const char * label, float * data, int dataSize, float minVal, float maxVal);
typedef void (*OdenGuiIndentFunc)(float indentW);
typedef void (*OdenGuiUnindentFunc)(float indentW);
typedef bool (*OdenColorEdit3Func)(const char * label, float color[3]);
typedef bool (*OdenColorEdit4Func)(const char * label, float color[4]);

// Entity functions

typedef struct OdenPluginEntityInitParams_s {
    void * entityUserData;

    // Settings
    OdenSettingsReadIntFunc readInt;
    OdenSettingsReadFloatFunc readFloat;
    OdenSettingsReadStringFunc readString;
    OdenSettingsWriteIntFunc writeInt;
    OdenSettingsWriteFloatFunc writeFloat;
    OdenSettingsWriteStringFunc writeString;
    OdenSettingsGetProjectPathFunc getProjectPath;
    OdenSettingsGetProjectPluginsPathFunc getProjectPluginsPath;
    OdenGetApplicationTypeFunc getApplicationType;
    OdenSetHasCrashedFunc setHasCrashed;
    OdenSetUiDisabledFunc setUiDisabled;
    OdenSettingsReadSceneStringFunc readSceneString;
    OdenSettingsWriteSceneStringFunc writeSceneString;
    OdenExitApplicationFunc exitApplication;
    OdenSettingsReadStringExFunc readStringEx;
    OdenSettingsGetProjectFilePathFunc getProjectFilePath;
    OdenGetOdenVersionFunc getOdenVersion;
    OdenGetLicenseHashFunc getLicenseHash;
    OdenGetInstanceUuidFunc getInstanceUuid;
    void * settingsReserved[3];

    // Registration
    OdenPluginRegisterImageCallbackFunc registerImageCallback;
    OdenPluginRegisterLidarCallbackFunc registerLidarCallback;
    OdenPluginRegisterShutdownCallbackFunc registerShutdownCallback;
    void * registrationReserved[15];

    // Upload functions
    OdenUploadImageFunc uploadImage;
    OdenUploadFontFunc uploadFont;
    OdenRegisterGlTexIdFunc registerGlTexId;
    OdenUploadImageExFunc uploadImageEx;
    OdenUploadShaderFunc uploadShader;
    OdenRegisterIOSurfaceIdFunc registerIOSurfaceId;
    void * uploadReserved[12];

    OdenGetGlLoaderFunc getGlLoader;
    OdenPublishData publishData;
    OdenQueryData queryData;
    OdenSetWindowTitleFunc setWindowTitle;

    void * reserved[124];

} OdenPluginEntityInitParams;

typedef struct OdenPluginEntityShutdownParams_s {
    void * entityUserData;

    // Settings
    OdenSettingsReadIntFunc readInt;
    OdenSettingsReadFloatFunc readFloat;
    OdenSettingsReadStringFunc readString;
    OdenSettingsWriteIntFunc writeInt;
    OdenSettingsWriteFloatFunc writeFloat;
    OdenSettingsWriteStringFunc writeString;
    OdenSettingsGetProjectPathFunc getProjectPath;
    OdenSettingsGetProjectPluginsPathFunc getProjectPluginsPath;
    OdenGetApplicationTypeFunc getApplicationType;
    OdenSetHasCrashedFunc setHasCrashed;
    OdenSetUiDisabledFunc setUiDisabled;
    OdenSettingsReadSceneStringFunc readSceneString;
    OdenSettingsWriteSceneStringFunc writeSceneString;
    OdenExitApplicationFunc exitApplication;
    OdenSettingsReadStringExFunc readStringEx;
    OdenSettingsGetProjectFilePathFunc getProjectFilePath;
    OdenPublishData publishData;
    OdenQueryData queryData;
    OdenGetOdenVersionFunc getOdenVersion;
    OdenGetLicenseHashFunc getLicenseHash;
    OdenGetInstanceUuidFunc getInstanceUuid;

    void * reserved[113];

} OdenPluginEntityShutdownParams;

typedef struct OdenPluginEntityUpdateParams_s {
    void * entityUserData;

    // Settings
    OdenSettingsReadIntFunc readInt;
    OdenSettingsReadFloatFunc readFloat;
    OdenSettingsReadStringFunc readString;
    OdenSettingsWriteIntFunc writeInt;
    OdenSettingsWriteFloatFunc writeFloat;
    OdenSettingsWriteStringFunc writeString;
    OdenSettingsGetProjectPathFunc getProjectPath;
    OdenSettingsGetProjectPluginsPathFunc getProjectPluginsPath;
    OdenGetApplicationTypeFunc getApplicationType;
    OdenSetHasCrashedFunc setHasCrashed;
    OdenSetUiDisabledFunc setUiDisabled;
    OdenSettingsReadSceneStringFunc readSceneString;
    OdenSettingsWriteSceneStringFunc writeSceneString;
    OdenExitApplicationFunc exitApplication;
    OdenSettingsReadStringExFunc readStringEx;
    OdenSettingsGetProjectFilePathFunc getProjectFilePath;
    OdenGetOdenVersionFunc getOdenVersion;
    OdenGetLicenseHashFunc getLicenseHash;
    void * settingsReserved[4];

    // Registration
    OdenPluginRegisterImageCallbackFunc registerImageCallback;
    OdenPluginRegisterLidarCallbackFunc registerLidarCallback;
    OdenPluginRegisterShutdownCallbackFunc registerShutdownCallback;
    void * registrationReserved[15];

    // Upload Image
    OdenUploadImageFunc uploadImage;
    OdenUploadFontFunc uploadFont;
    OdenRegisterGlTexIdFunc registerGlTexId;
    OdenUploadImageExFunc uploadImageEx;
    OdenUploadShaderFunc uploadShader;
    OdenRegisterIOSurfaceIdFunc registerIOSurfaceId;

    // Graphics functions
    OdenGetGlLoaderFunc getGlLoader;
    OdenGetCameraGlTexIdFunc getCameraGlTexId;
    void * graphicsReserved[10];

    // Update
    OdenUpdateRecordingPushDataFunc pushRecordingData;

    OdenUpdateResetViewFunc resetView;
    OdenUpdateSetFpsCameraAnglesAndPositionFunc setFpsCameraAnglesAndPosition;
    OdenUpdateGetMouseRayFunc getMouseRay;
    OdenUpdateGetCameraForwardRayFunc getCameraForwardRay;
    OdenUpdateIsMousePressedSinceLastSwapFunc isMousePressedSinceLastSwap;
    OdenUpdateGetViewportMousePosFunc getViewportMousePos;
    OdenUpdateIsMouseReleasedSinceLastSwapFunc isMouseReleasedSinceLastSwap;
    OdenUpdateDownloadVirtualCameraImage downloadVirtualCameraImage;
    OdenUpdateVirtualCameraPixelToRay virtualCameraPixelToRay;
    OdenUpdateGetMonitorInfo getMonitorInfo;
    OdenUpdateGetViewportInfoFunc getViewportInfo;
    OdenUpdateGetViewMousePosPxWithOffsetFunc getViewMousePosPxWithOffset;
    void * updateReserved[120];

    // ImDraw
    OdenUpdateImDrawSetUseZBufferFunc imDrawSetUseZBuffer;
    OdenUpdateImDrawSetWriteZBufferFunc imDrawSetWriteZBuffer;
    OdenUpdateImDrawPushMatrixFunc imDrawPushMatrix;
    OdenUpdateImDrawPopMatrixFunc imDrawPopMatrix;
    OdenUpdateImDrawPushViewSpace imDrawPushViewSpace;
    OdenUpdateImDrawPopViewSpace imDrawPopViewSpace;
    OdenUpdateImDrawSetViewSpaceZ imDrawSetViewSpaceZ;
    OdenUpdateImDrawSetTint imDrawSetTint;
    OdenUpdateImDrawPushShaderFunc imDrawPushShader;
    OdenUpdateImDrawPopShaderFunc imDrawPopShader;
    OdenUpdateImDrawLatchBackbufferFunc imDrawLatchBackbuffer;
    void * imDrawReservedState[9];

    OdenUpdateImDrawAddLineFunc imDrawAddLine;
    OdenUpdateImDrawAddCircleSegmentFunc imDrawAddCircleSegment;
    OdenUpdateImDrawAddCircleFunc imDrawAddCircle;
    OdenUpdateImDrawAddSquareFunc imDrawAddSquare;
    OdenUpdateImDrawAddRectangleFunc imDrawAddRectangle;
    OdenUpdateImDrawAddRectangleRoundedFunc imDrawAddRectangleRounded;
    OdenUpdateImDrawAddImageFunc imDrawAddImage;
    OdenUpdateImDrawAddTextFunc imDrawAddText;
    OdenUpdateImDrawAddTextNormalFunc imDrawAddTextNormal;
    OdenUpdateImDrawAddTextAlignedFunc imDrawAddTextAligned;
    OdenUpdateImDrawAddTextAlignedNormalFunc imDrawAddTextAlignedNormal;
    OdenUpdateImDrawAddTorusFunc imDrawAddTorus;
    OdenUpdateImDrawAddLinesFunc imDrawAddLines;
    OdenUpdateImDrawAddRectangleBorderFunc imDrawAddRectangleBorder;
    OdenUpdateImDrawAddRectangleBorderRoundedFunc imDrawAddRectangleBorderRounded;
    OdenUpdateImDrawAddRectangleAlignedFunc imDrawAddRectangleAligned;
    OdenUpdateImDrawAddRectangleRoundedAlignedFunc imDrawAddRectangleRoundedAligned;
    OdenUpdateImDrawAddRectangleBorderAlignedFunc imDrawAddRectangleBorderAligned;
    OdenUpdateImDrawAddRectangleBorderRoundedAlignedFunc imDrawAddRectangleBorderRoundedAligned;
    OdenUpdateImDrawAddCuboidFunc imDrawAddCuboid;
    OdenUpdateImDrawAddTrianglesFunc imDrawAddTriangles;
    OdenUpdateImDrawPushScreenSpace imDrawPushScreenSpace;
    OdenUpdateImDrawPopScreenSpace imDrawPopScreenSpace;
    OdenUpdateImDrawAddVideoFunc imDrawAddVideo;
    OdenUpdateImDrawAddTrianglesWithUvsFunc imDrawAddTrianglesWithUvs;
    OdenUpdateImDrawAddVideoWithUvsFunc imDrawAddVideoWithUvs;
    void * imDrawReservedAdd[50];

    OdenUpdateImDrawIsMouseOverLastElementFunc imDrawIsMouseOverLastElement;
    OdenUpdateImDrawIsMouseClickedLastElementFunc isMouseClickedLastElement;
    OdenUpdateImDrawIsMousePressedOverLastElementFunc isMousePressedOverLastElement;
    OdenUpdateImDrawRemoveLastElementFunc imDrawRemoveLastElement;
    OdenUpdateimDrawCalcTextSizeFunc imDrawCalcTextSize;
    void * imDrawReservedIs[15];

    // Scene
    OdenGetHorizonFunc getHorizon;
    OdenSetHorizonFunc setHorizon;
    OdenSetSeparateProjectionsFunc setSeparateProjections;
    OdenSetCurtainFunc setCurtain;
    OdenSetStitchDistanceFunc setStitchDistance;
    OdenSetCameraBrightnessFunc setCameraBrightness;
    union {
        DEPRECATED("Rename to reflect that it works for GigE cameras too")
        OdenGetAllConnectedGigECameraSerialsFunc getAllConnectedIdsCameraSerials;
        OdenGetAllConnectedGigECameraSerialsFunc getAllConnectedGigECameraSerials;
    };
    OdenSetPanoramaVideoSettingsStringFunc setPanoramaVideoSettingsString;
    OdenSetCameraCalibrationStringFunc setCameraCalibrationString;
    OdenGetCameraCalibrationFunc getCameraCalibration;
    OdenSetCameraCalibrationFunc setCameraCalibration;
    OdenGetCameraCropDataFunc getCameraCropData;
    OdenGetCameraMetadataFunc getCameraMetadata;
    OdenGetCameraTemperatureFunc getCameraTemperature;
    OdenGetCameraFpsFunc getCameraFps;
    OdenGetCameraLastValidTimestampFunc getCameraLastValidTimestamp;
    OdenSetEntityEnabledFunc setEntityEnabled;
    OdenSetEntityTransformFunc setEntityTransform;
    OdenSetVirtualCameraPositionFunc setVirtualCameraPosition;
    OdenSetVirtualCameraRotationFunc setVirtualCameraRotation;
    OdenSetVirtualCameraFovFunc setVirtualCameraFov;
    OdenSetVirtualCameraResolutionFunc setVirtualCameraResolution;
    OdenSetVirtualCameraCircularCropFunc setVirtualCameraCircularCrop;
    OdenGetFaultsFunc getFaults;
    OdenShowHmdManualAdjustmentPatternFunc showHmdManualAdjustmentPattern;
    OdenInhibitMouseLookFunc inhibitMouseLook;
    OdenPublishData publishData;
    OdenQueryData queryData;
    OdenHasEntityWithNameFunc hasEntityWithName;
    OdenGetEntityWorldMatrixFunc getEntityWorldMatrix;
    OdenGetPanoramaVideoSettingsStringFunc getPanoramaVideoSettingsString;
    OdenGetCameraRigSettingsStringFunc getCameraRigSettingsString;
    OdenGetCameraCurrentTimestampFunc getCameraCurrentTimestamp;
    OdenIsJoystickPresentFunc isJoystickPresent;
    OdenGetJoystickStateFunc getJoystickState;
    DEPRECATED("Traffic can come from several ip addresses, this function will give you the first one")
    OdenGetStreamSenderIpFunc getStreamSenderIp;
    OdenGetStreamStatisticsFunc getStreamStatistics;
    OdenGetAllSceneIdsFunc getAllSceneIds;
    OdenGetSelectedSceneIdFunc getSelectedSceneId;
    OdenSetSelectedSceneFunc setSelectedScene;
    OdenGetSceneNameFunc getSceneName;
    OdenRawRecordStartFunc rawRecordStart;
    OdenRawRecordStopFunc rawRecordStop;
    OdenRawRecordSetEntityRingBufferSizeFunc rawRecordSetEntityRingBufferSize;
    OdenRawRecordEntityRingBufferEnableFunc rawRecordEntityRingBufferEnable;
    OdenPlaybackIsPlayingFunc playbackIsPlaying;
    OdenPlaybackStartFunc playbackStart;
    OdenPlaybackPauseFunc playbackPause;
    OdenPlaybackGetTimesFunc playbackGetTimes;
    OdenPlaybackSetTimeFunc playbackSetTime;
    OdenPlaybackAdvanceTimeFunc playbackAdvanceTime;
    OdenPlaybackSetLoopFunc playbackSetLoop;
    DEPRECATED("Use the link functions instead")
    OdenGetStreamerDestinationIpFunc getStreamerDestinationIp;
    union {
        DEPRECATED("Rename to reflect that it works for GigE cameras too")
        OdenGetGigECameraSerial getIdsCameraSerial;
        OdenGetGigECameraSerial getGigECameraSerial;
    };
    OdenSetStreamerFrameMetadata setStreamerFrameMetadata;
    OdenSetCameraResolutionFunc setCameraResolution;
    OdenSetCameraGammaFunc setCameraGamma;
    OdenSetCameraAutoExposureRoiFunc setCameraAutoExposureRoi;
    OdenLoadProjectFunc loadProject;
    OdenSaveProjectFunc saveProject;
    OdenProjectHasUnsavedChangesFunc projectHasUnsavedChanges;
    OdenModelEntitySetPathFunc modelEntitySetPath;
    OdenSetStreamerTargetChannelUtilizationFunc setStreamerTargetChannelUtilization;
    OdenSetStreamerMaxBandwidthFunc setStreamerMaxBandwidth;
    OdenSetCameraStreamStateFunc setCameraStreamState;
    OdenGetCameraStreamStateFunc getCameraStreamState;
    OdenSetForceFeedbackAngleFunc setForceFeedbackAngle;
    OdenGetStreamStatisticsExFunc getStreamStatisticsEx;
    OdenGetStreamerBitrateMbpsFunc getStreamerBitrateMbps;
    OdenStartStreamerFunc startStreamer;
    OdenStopStreamerFunc stopStreamer;
    OdenIsStreamerRunningFunc isStreamerRunning;
    DEPRECATED("Use the link functions instead")
    OdenSetStreamerDestinationIpFunc setStreamerDestinationIp;
    DEPRECATED("Use the link functions instead")
    OdenSetStreamerDestinationPortFunc setStreamerDestinationPort;
    DEPRECATED("Use the link functions instead")
    OdenGetStreamerDestinationPortFunc getStreamerDestinationPort;
    DEPRECATED("Use the link functions instead")
    OdenSetStreamerAudioDestinationPortFunc setStreamerAudioDestinationPort;
    DEPRECATED("Use the link functions instead")
    OdenGetStreamerAudioDestinationPortFunc getStreamerAudioDestinationPort;
    DEPRECATED("Use the link functions instead")
    OdenSetStreamerFeedbackDestinationPortFunc setStreamerFeedbackDestinationPort;
    DEPRECATED("Use the link functions instead")
    OdenGetStreamerFeedbackDestinationPortFunc getStreamerFeedbackDestinationPort;
    DEPRECATED("Use the link functions instead")
    OdenSetComChannelDestinationPortFunc setComChannelDestinationPort;
    DEPRECATED("Use the link functions instead")
    OdenGetComChannelDestinationPortFunc getComChannelDestinationPort;
    DEPRECATED("Use the link functions instead")
    OdenSetComChannelReceivePortFunc setComChannelReceivePort;
    DEPRECATED("Use the link functions instead")
    OdenGetComChannelReceivePortFunc getComChannelReceivePort;
    DEPRECATED("Use the link functions instead")
    OdenSetVideoReceivePortFunc setVideoReceivePort;
    DEPRECATED("Use the link functions instead")
    OdenGetVideoReceivePortFunc getVideoReceivePort;
    DEPRECATED("Use the link functions instead")
    OdenSetRealtimeAudioLinkReceivePortFunc setRealtimeAudioLinkReceivePort;
    DEPRECATED("Use the link functions instead")
    OdenGetRealtimeAudioLinkReceivePortFunc getRealtimeAudioLinkReceivePort;
    DEPRECATED("Use the link functions instead")
    OdenSetRemoteStreamerFeedbackReceivePortFunc setRemoteStreamerFeedbackReceivePort;
    DEPRECATED("Use the link functions instead")
    OdenGetRemoteStreamerFeedbackReceivePortFunc getRemoteStreamerFeedbackReceivePort;
    OdenRawRecordIsRunningFunc rawRecordIsRunning;
    DEPRECATED("No-op: Producer outputs no longer exist")
    OdenStartEncodedProducerOutputsFunc startEncodedProducerOutputs;
    OdenApplyProjectTemplateFunc applyProjectTemplate;
    OdenGetLinkCountFunc getLinkCount;
    OdenSetLinkModeFunc setLinkMode;
    OdenGetLinkModeFunc getLinkMode;
    OdenSetLinkBindIpFunc setLinkBindIp;
    OdenGetLinkBindIpFunc getLinkBindIp;
    OdenSetLinkReceivePortFunc setLinkReceivePort;
    OdenGetLinkReceivePortFunc getLinkReceivePort;
    OdenSetLinkDestinationIpFunc setLinkDestinationIp;
    OdenGetLinkDestinationIpFunc getLinkDestinationIp;
    OdenSetLinkDestinationPortFunc setLinkDestinationPort;
    OdenGetLinkDestinationPortFunc getLinkDestinationPort;
    OdenGetAllEntityUuidsFunc getAllEntityUuids;
    OdenGetEntityTypeNameFunc getEntityTypeName;
    OdenGetBasicStatisticsFunc getBasicStatistics;
    OdenSetLinkEncryptionPrivateKeyPathFunc setLinkEncryptionPrivateKeyPath;
    OdenGetLinkEncryptionPrivateKeyPathFunc getLinkEncryptionPrivateKeyPath;
    OdenAddLinkEncryptionAllowedPublicKeyFunc addLinkEncryptionAllowedPublicKey;
    OdenRemoveLinkEncryptionAllowedPublicKeyFunc removeLinkEncryptionAllowedPublicKey;
    OdenGetLinkEncryptionAllowedPublicKeysCountFunc getLinkEncryptionAllowedPublicKeysCount;
    OdenGetLinkEncryptionAllowedPublicKeyFunc getLinkEncryptionAllowedPublicKey;
    DEPRECATED("No-op: Producer removed. Kept for ABI compatibility. Always returns false.")
    OdenGetLiveSceneIdFunc getLiveSceneId;
    DEPRECATED("No-op: Producer removed. Kept for ABI compatibility. Always returns false.")
    OdenSetLiveSceneFunc setLiveScene;
    OdenShowHmdControllersFunc showHmdControllers;
    OdenSetFocusRegionCenterPixelPositionFunc setFocusRegionCenterPixelPosition;
    OdenGetEntityVideoSizeFunc getEntityVideoSize;
    OdenSetRegulatorModeFunc setRegulatorMode;
    OdenGetRegulatorModeFunc getRegulatorMode;
    OdenSetForceFeedbackForceFunc setForceFeedbackForce;
    OdenSetCloneStreamSourceFunc setCloneStreamSource;
    OdenGetSceneContainingPluginEntityFunc getSceneContainingPluginEntity;
    OdenSetLinkEncryptionInternalSourceIpFunc setLinkEncryptionInternalSourceIp;
    OdenGetLinkEncryptionInternalSourceIpFunc getLinkEncryptionInternalSourceIp;
    OdenSetLinkEncryptionInternalDestinationIpFunc setLinkEncryptionInternalDestinationIp;
    OdenGetLinkEncryptionInternalDestinationIpFunc getLinkEncryptionInternalDestinationIp;
    OdenAddLinkFunc addLink;
    OdenRemoveLinkFunc removeLink;
    OdenGetInstanceUuidFunc getInstanceUuid;
    OdenSetLinkP2pEnabledFunc setLinkP2pEnabled;
    OdenGetLinkP2pEnabledFunc getLinkP2pEnabled;
    OdenGetLinkP2pStatusFunc getLinkP2pStatus;
    OdenSetLinkP2pStunServerFunc setLinkP2pStunServer;
    OdenGetLinkP2pStunServerFunc getLinkP2pStunServer;
    OdenGetCalibratedFloorHeightFunc getCalibratedFloorHeight;
    OdenGetEntityNameFunc getEntityName;
    OdenGetCameraLastReceivedFrameTimeFunc getCameraLastReceivedFrameTime;
    OdenGetAlignmentIdFunc getAlignmentId;
    OdenGetVideoPlacementFunc getVideoPlacement;
    DEPRECATED("Not supported")
    OdenGetVirtualWebcamNameFunc getVirtualWebcamName;
    OdenUpdateSetAudioReceivingEnabled setAudioReceivingEnabled;
    OdenUpdateSetAudioSendingEnabled setAudioSendingEnabled;
    OdenRawRecordGetLastPath rawRecordGetLastPath;
    OdenLoadSceneFunc loadScene;
    OdenRemoveSceneFunc removeScene;
    OdenGetLinkStatisticsFunc getLinkStatistics;
    OdenGetSceneParamFunc getSceneParam;
    OdenSetSceneParamFunc setSceneParam;
    OdenCloseProjectFunc closeProject;
    OdenAddEntityFunc addEntity;
    OdenRemoveEntityFunc removeEntity;
    OdenPollRemoteCalibrationFunc pollRemoteCalibration;
    OdenSetExternalStitchGeometryFunc setExternalStitchGeometry;
    OdenGetParentUuidFunc getParentUuid;
    OdenGetEntityUuidFunc getEntityUuid;
    OdenGetGamepadStateFunc getGamepadState;

    // Camera
    OdenSetFpsCameraFieldOfViewFunc setFpsCameraFieldOfView;
    OdenSetHmdCameraFieldOfViewFactorFunc setHmdCameraFieldOfViewFactor;
    OdenGetSavedCameraStatesFunc getSavedCameraStates;
    OdenGetSavedCameraStateFunc getSavedCameraState;
    OdenSetSavedCameraStateFunc setSavedCameraState;
    OdenLerpToSavedCameraStateFunc lerpToSavedCameraState;
    OdenGetFpsCameraStateFunc getFpsCameraState;
    OdenGetFpsCameraTypeFunc getFpsCameraType;
    OdenSetFpsCameraTypeFunc setFpsCameraType;
    OdenSetGuiVisibleFunc setGuiVisible;
    OdenPushHmdPostTransformFunc pushHmdPostTransform;
    OdenGetHmdTransformFunc getHmdTransform;

    void * cameraReserved[10];

    // Input
    OdenInputIsKeyDownFunc isKeyDown;
    OdenInputIsKeyPressedSinceLastSwapFunc isKeyPressedSinceLastSwap;
    OdenInputGetKeysPressedSinceLastSwap getKeysPressedSinceLastSwap;
    OdenInputGetWindowEventsFunc getWindowEvents;
    OdenGetGamepadStateV2Func getGamepadStateV2;

    void * inputReserved[2];

    // Messaging
    OdenComChannelMaxMessageSizeFunc comChannelMaxMessageSize;
    OdenComChannelSendMessageFunc comChannelSendMessage;
    OdenComChannelGetMessageCountFunc comChannelGetMessageCount;
    OdenComChannelGetMessageFunc comChannelGetMessage;
    OdenComChannelGetLastMessageFunc comChannelGetLastMessage;

    OdenComChannelGetVarSizeMsgFunc comChannelGetVarSizeMsg;
    OdenComChannelGetLastVarSizeMsgFunc comChannelGetLastVarSizeMsg;

    void * messagingReserved[10];

    // Extended
    OdenSetMouseCursorFunc setMouseCursor;
    OdenSetWindowTitleFunc setWindowTitle;
    OdenGetDewarpingParamsFunc getDewarpingParams;
    OdenSetDewarpingParamsFunc setDewarpingParams;
    OdenPreventCloseFunc preventClose;
    OdenCloseWasDeniedFunc closeWasDenied;

    OdenConfigureTextureStreamingFunc configureTextureStreaming;
    OdenStartTextureStreamingFunc startTextureStreaming;
    OdenStopTextureStreamingFunc stopTextureStreaming;
    OdenGetStreamerLinkFeedbackStatsFunc getStreamerLinkFeedbackStats;
    OdenGetStreamPacketLossFunc getStreamPacketLoss;
    OdenSetLinkAllowOutgoingFeedbackFunc setLinkAllowOutgoingFeedback;
    void * reserved[178];

    // Link EXT
    OdenClearLinkEncryptionAllowedPublicKeysFunc clearLinkEncryptionAllowedPublicKeys;
    OdenSetLinkToRelayLinkFunc setLinkToRelayLink;
    OdenIsLinkRelayLinkFunc isLinkRelayLink;
    OdenSetLinkSendToRelayFunc setLinkSendToRelay;
    OdenIsLinkSendingToRelayFunc isLinkSendingToRelay;
    OdenSetLinkDropOutgoingPackets setLinkDropOutgoingPackets;
    OdenAddLinkP2pExcludedSubnetFunc addLinkP2pExcludedSubnet;
    OdenClearLinkP2pExcludedSubnetsFunc clearLinkP2pExcludedSubnets;
    OdenGetLinkTimeSinceLastReceivedPacketNsFunc getLinkTimeSinceLastReceivedPacketNs;
    void * linkReserved;

    // Neural networks
    OdenAddNeuralNetworkFunc addNeuralNetwork;
    OdenRemoveNeuralNetworkFunc removeNeuralNetwork;
    OdenSetNeuralNetworkModelFunc setNeuralNetworkModel;
    OdenSetNeuralNetworkInputVideoStreamFunc setNeuralNetworkInputVideoStream;
    OdenGetNeuralNetworkTensorDataFunc getNeuralNetworkTensorData;
    OdenTriggerNeuralNetworkRunFunc triggerNeuralNetworkRun;
    OdenGetNeuralNetworkInfoFunc getNeuralNetworkInfo;
    OdenGetInferenceSettingsFunc getInferenceSettings;
    OdenSetNeuralNetworkDlaCoreFunc setNeuralNetworkDlaCore;
    void * neuralNetworksReserved[7];

    // Parameters
    OdenMatrix4 worldMatrix;
    double dt;

    void * dataReserved[31];

} OdenPluginEntityUpdateParams;

typedef struct OdenPluginEntityDrawParams_s {
    void * entityUserData;

    // Settings
    OdenSettingsReadIntFunc readInt;
    OdenSettingsReadFloatFunc readFloat;
    OdenSettingsReadStringFunc readString;
    OdenSettingsWriteIntFunc writeInt;
    OdenSettingsWriteFloatFunc writeFloat;
    OdenSettingsWriteStringFunc writeString;
    OdenSettingsGetProjectPathFunc getProjectPath;
    OdenSettingsGetProjectPluginsPathFunc getProjectPluginsPath;
    OdenGetApplicationTypeFunc getApplicationType;
    OdenSetHasCrashedFunc setHasCrashed;
    OdenSetUiDisabledFunc setUiDisabled;
    OdenGetOdenVersionFunc getOdenVersion;

    // Graphics functions
    OdenGetGlLoaderFunc getGlLoader;
    OdenGetCameraGlTexIdFunc getCameraGlTexId;
    void * graphicsReserved[8];

    // Scene
    OdenGetHorizonFunc getHorizon;
    OdenSetHorizonFunc setHorizon;
    OdenSetSeparateProjectionsFunc setSeparateProjections;
    OdenSetCurtainFunc setCurtain;
    OdenSetStitchDistanceFunc setStitchDistance;
    OdenSetCameraBrightnessFunc setCameraBrightness;
    union {
        DEPRECATED("Rename to reflect that it works for GigE cameras too")
        OdenGetAllConnectedGigECameraSerialsFunc getAllConnectedIdsCameraSerials;
        OdenGetAllConnectedGigECameraSerialsFunc getAllConnectedGigECameraSerials;
    };
    OdenSetPanoramaVideoSettingsStringFunc setPanoramaVideoSettingsString;
    OdenSetCameraCalibrationStringFunc setCameraCalibrationString;
    OdenGetCameraCalibrationFunc getCameraCalibration;
    OdenSetCameraCalibrationFunc setCameraCalibration;
    OdenGetCameraCropDataFunc getCameraCropData;
    OdenGetCameraMetadataFunc getCameraMetadata;
    OdenGetCameraTemperatureFunc getCameraTemperature;
    OdenGetCameraFpsFunc getCameraFps;
    OdenGetCameraLastValidTimestampFunc getCameraLastValidTimestamp;
    OdenSetEntityEnabledFunc setEntityEnabled;
    OdenSetEntityTransformFunc setEntityTransform;
    OdenSetVirtualCameraPositionFunc setVirtualCameraPosition;
    OdenSetVirtualCameraRotationFunc setVirtualCameraRotation;
    OdenSetVirtualCameraFovFunc setVirtualCameraFov;
    OdenSetVirtualCameraResolutionFunc setVirtualCameraResolution;
    OdenSetVirtualCameraCircularCropFunc setVirtualCameraCircularCrop;
    OdenGetFaultsFunc getFaults;
    OdenShowHmdManualAdjustmentPatternFunc showHmdManualAdjustmentPattern;
    OdenInhibitMouseLookFunc inhibitMouseLook;
    OdenPublishData publishData;
    OdenQueryData queryData;
    OdenHasEntityWithNameFunc hasEntityWithName;
    OdenGetEntityWorldMatrixFunc getEntityWorldMatrix;
    OdenGetPanoramaVideoSettingsStringFunc getPanoramaVideoSettingsString;
    OdenGetCameraRigSettingsStringFunc getCameraRigSettingsString;
    OdenGetCameraCurrentTimestampFunc getCameraCurrentTimestamp;
    OdenIsJoystickPresentFunc isJoystickPresent;
    OdenGetJoystickStateFunc getJoystickState;
    DEPRECATED("Traffic can come from several ip addresses, this function will give you the first one")
    OdenGetStreamSenderIpFunc getStreamSenderIp;
    OdenGetStreamStatisticsFunc getStreamStatistics;
    OdenGetAllSceneIdsFunc getAllSceneIds;
    OdenGetSelectedSceneIdFunc getSelectedSceneId;
    OdenSetSelectedSceneFunc setSelectedScene;
    OdenGetSceneNameFunc getSceneName;
    OdenRawRecordStartFunc rawRecordStart;
    OdenRawRecordStopFunc rawRecordStop;
    OdenRawRecordSetEntityRingBufferSizeFunc rawRecordSetEntityRingBufferSize;
    OdenRawRecordEntityRingBufferEnableFunc rawRecordEntityRingBufferEnable;
    OdenPlaybackIsPlayingFunc playbackIsPlaying;
    OdenPlaybackStartFunc playbackStart;
    OdenPlaybackPauseFunc playbackPause;
    OdenPlaybackGetTimesFunc playbackGetTimes;
    OdenPlaybackSetTimeFunc playbackSetTime;
    OdenPlaybackAdvanceTimeFunc playbackAdvanceTime;
    OdenPlaybackSetLoopFunc playbackSetLoop;
    DEPRECATED("Use the link functions instead")
    OdenGetStreamerDestinationIpFunc getStreamerDestinationIp;
    union {
        DEPRECATED("Rename to reflect that it works for GigE cameras too")
        OdenGetGigECameraSerial getIdsCameraSerial;
        OdenGetGigECameraSerial getGigECameraSerial;
    };
    OdenSetStreamerFrameMetadata setStreamerFrameMetadata;
    OdenSetCameraResolutionFunc setCameraResolution;
    OdenSetCameraGammaFunc setCameraGamma;
    OdenSetCameraAutoExposureRoiFunc setCameraAutoExposureRoi;
    OdenLoadProjectFunc loadProject;
    OdenSaveProjectFunc saveProject;
    OdenProjectHasUnsavedChangesFunc projectHasUnsavedChanges;
    OdenModelEntitySetPathFunc modelEntitySetPath;
    OdenSetStreamerTargetChannelUtilizationFunc setStreamerTargetChannelUtilization;
    OdenSetStreamerMaxBandwidthFunc setStreamerMaxBandwidth;
    OdenSetCameraStreamStateFunc setCameraStreamState;
    OdenGetCameraStreamStateFunc getCameraStreamState;
    OdenSetForceFeedbackAngleFunc setForceFeedbackAngle;
    OdenGetStreamStatisticsExFunc getStreamStatisticsEx;
    OdenGetStreamerBitrateMbpsFunc getStreamerBitrateMbps;
    OdenStartStreamerFunc startStreamer;
    OdenStopStreamerFunc stopStreamer;
    OdenIsStreamerRunningFunc isStreamerRunning;
    DEPRECATED("Use the link functions instead")
    OdenSetStreamerDestinationIpFunc setStreamerDestinationIp;
    DEPRECATED("Use the link functions instead")
    OdenSetStreamerDestinationPortFunc setStreamerDestinationPort;
    DEPRECATED("Use the link functions instead")
    OdenGetStreamerDestinationPortFunc getStreamerDestinationPort;
    DEPRECATED("Use the link functions instead")
    OdenSetStreamerAudioDestinationPortFunc setStreamerAudioDestinationPort;
    DEPRECATED("Use the link functions instead")
    OdenGetStreamerAudioDestinationPortFunc getStreamerAudioDestinationPort;
    DEPRECATED("Use the link functions instead")
    OdenSetStreamerFeedbackDestinationPortFunc setStreamerFeedbackDestinationPort;
    DEPRECATED("Use the link functions instead")
    OdenGetStreamerFeedbackDestinationPortFunc getStreamerFeedbackDestinationPort;
    DEPRECATED("Use the link functions instead")
    OdenSetComChannelDestinationPortFunc setComChannelDestinationPort;
    DEPRECATED("Use the link functions instead")
    OdenGetComChannelDestinationPortFunc getComChannelDestinationPort;
    DEPRECATED("Use the link functions instead")
    OdenSetComChannelReceivePortFunc setComChannelReceivePort;
    DEPRECATED("Use the link functions instead")
    OdenGetComChannelReceivePortFunc getComChannelReceivePort;
    DEPRECATED("Use the link functions instead")
    OdenSetVideoReceivePortFunc setVideoReceivePort;
    DEPRECATED("Use the link functions instead")
    OdenGetVideoReceivePortFunc getVideoReceivePort;
    DEPRECATED("Use the link functions instead")
    OdenSetRealtimeAudioLinkReceivePortFunc setRealtimeAudioLinkReceivePort;
    DEPRECATED("Use the link functions instead")
    OdenGetRealtimeAudioLinkReceivePortFunc getRealtimeAudioLinkReceivePort;
    DEPRECATED("Use the link functions instead")
    OdenSetRemoteStreamerFeedbackReceivePortFunc setRemoteStreamerFeedbackReceivePort;
    DEPRECATED("Use the link functions instead")
    OdenGetRemoteStreamerFeedbackReceivePortFunc getRemoteStreamerFeedbackReceivePort;
    OdenRawRecordIsRunningFunc rawRecordIsRunning;
    DEPRECATED("No-op: Producer outputs no longer exist")
    OdenStartEncodedProducerOutputsFunc startEncodedProducerOutputs;
    OdenApplyProjectTemplateFunc applyProjectTemplate;
    OdenGetLinkCountFunc getLinkCount;
    OdenSetLinkModeFunc setLinkMode;
    OdenGetLinkModeFunc getLinkMode;
    OdenSetLinkBindIpFunc setLinkBindIp;
    OdenGetLinkBindIpFunc getLinkBindIp;
    OdenSetLinkReceivePortFunc setLinkReceivePort;
    OdenGetLinkReceivePortFunc getLinkReceivePort;
    OdenSetLinkDestinationIpFunc setLinkDestinationIp;
    OdenGetLinkDestinationIpFunc getLinkDestinationIp;
    OdenSetLinkDestinationPortFunc setLinkDestinationPort;
    OdenGetLinkDestinationPortFunc getLinkDestinationPort;
    OdenGetAllEntityUuidsFunc getAllEntityUuids;
    OdenGetEntityTypeNameFunc getEntityTypeName;
    OdenGetBasicStatisticsFunc getBasicStatistics;
    OdenSetLinkEncryptionPrivateKeyPathFunc setLinkEncryptionPrivateKeyPath;
    OdenGetLinkEncryptionPrivateKeyPathFunc getLinkEncryptionPrivateKeyPath;
    OdenAddLinkEncryptionAllowedPublicKeyFunc addLinkEncryptionAllowedPublicKey;
    OdenRemoveLinkEncryptionAllowedPublicKeyFunc removeLinkEncryptionAllowedPublicKey;
    OdenGetLinkEncryptionAllowedPublicKeysCountFunc getLinkEncryptionAllowedPublicKeysCount;
    OdenGetLinkEncryptionAllowedPublicKeyFunc getLinkEncryptionAllowedPublicKey;
    DEPRECATED("No-op: Producer removed. Kept for ABI compatibility. Always returns false.")
    OdenGetLiveSceneIdFunc getLiveSceneId;
    DEPRECATED("No-op: Producer removed. Kept for ABI compatibility. Always returns false.")
    OdenSetLiveSceneFunc setLiveScene;
    OdenShowHmdControllersFunc showHmdControllers;
    OdenSetFocusRegionCenterPixelPositionFunc setFocusRegionCenterPixelPosition;
    OdenGetEntityVideoSizeFunc getEntityVideoSize;
    OdenSetRegulatorModeFunc setRegulatorMode;
    OdenGetRegulatorModeFunc getRegulatorMode;
    OdenSetForceFeedbackForceFunc setForceFeedbackForce;
    OdenSetCloneStreamSourceFunc setCloneStreamSource;
    OdenGetSceneContainingPluginEntityFunc getSceneContainingPluginEntity;
    OdenSetLinkEncryptionInternalSourceIpFunc setLinkEncryptionInternalSourceIp;
    OdenGetLinkEncryptionInternalSourceIpFunc getLinkEncryptionInternalSourceIp;
    OdenSetLinkEncryptionInternalDestinationIpFunc setLinkEncryptionInternalDestinationIp;
    OdenGetLinkEncryptionInternalDestinationIpFunc getLinkEncryptionInternalDestinationIp;
    OdenAddLinkFunc addLink;
    OdenRemoveLinkFunc removeLink;
    OdenGetInstanceUuidFunc getInstanceUuid;
    OdenSetLinkP2pEnabledFunc setLinkP2pEnabled;
    OdenGetLinkP2pEnabledFunc getLinkP2pEnabled;
    OdenGetLinkP2pStatusFunc getLinkP2pStatus;
    OdenSetLinkP2pStunServerFunc setLinkP2pStunServer;
    OdenGetLinkP2pStunServerFunc getLinkP2pStunServer;
    OdenGetCalibratedFloorHeightFunc getCalibratedFloorHeight;
    OdenGetEntityNameFunc getEntityName;
    OdenGetCameraLastReceivedFrameTimeFunc getCameraLastReceivedFrameTime;
    OdenGetAlignmentIdFunc getAlignmentId;
    OdenGetVideoPlacementFunc getVideoPlacement;
    DEPRECATED("Not supported")
    OdenGetVirtualWebcamNameFunc getVirtualWebcamName;
    OdenUpdateSetAudioReceivingEnabled setAudioReceivingEnabled;
    OdenUpdateSetAudioSendingEnabled setAudioSendingEnabled;
    OdenRawRecordGetLastPath rawRecordGetLastPath;
    OdenLoadSceneFunc loadScene;
    OdenRemoveSceneFunc removeScene;
    OdenGetLinkStatisticsFunc getLinkStatistics;
    OdenGetSceneParamFunc getSceneParam;
    OdenSetSceneParamFunc setSceneParam;
    OdenCloseProjectFunc closeProject;
    OdenAddEntityFunc addEntity;
    OdenRemoveEntityFunc removeEntity;
    OdenPollRemoteCalibrationFunc pollRemoteCalibration;
    OdenSetExternalStitchGeometryFunc setExternalStitchGeometry;
    OdenGetParentUuidFunc getParentUuid;
    OdenGetEntityUuidFunc getEntityUuid;
    OdenGetGamepadStateFunc getGamepadState;

    // Camera
    OdenSetFpsCameraFieldOfViewFunc setFpsCameraFieldOfView;
    OdenSetHmdCameraFieldOfViewFactorFunc setHmdCameraFieldOfViewFactor;
    OdenGetSavedCameraStatesFunc getSavedCameraStates;
    OdenGetSavedCameraStateFunc getSavedCameraState;
    OdenSetSavedCameraStateFunc setSavedCameraState;
    OdenLerpToSavedCameraStateFunc lerpToSavedCameraState;
    OdenGetFpsCameraStateFunc getFpsCameraState;
    OdenGetFpsCameraTypeFunc getFpsCameraType;
    OdenSetFpsCameraTypeFunc setFpsCameraType;
    OdenSetGuiVisibleFunc setGuiVisible;
    OdenPushHmdPostTransformFunc pushHmdPostTransform;
    OdenGetHmdTransformFunc getHmdTransform;

    void * cameraReserved[10];

    // Input
    OdenInputIsKeyDownFunc isKeyDown;
    OdenInputIsKeyPressedSinceLastSwapFunc isKeyPressedSinceLastSwap;
    OdenInputGetKeysPressedSinceLastSwap getKeysPressedSinceLastSwap;
    OdenInputGetWindowEventsFunc getWindowEvents;
    OdenGetGamepadStateV2Func getGamepadStateV2;

    void * inputReserved[2];

    // Messaging
    OdenComChannelMaxMessageSizeFunc comChannelMaxMessageSize;
    OdenComChannelSendMessageFunc comChannelSendMessage;
    OdenComChannelGetMessageCountFunc comChannelGetMessageCount;
    OdenComChannelGetMessageFunc comChannelGetMessage;
    OdenComChannelGetLastMessageFunc comChannelGetLastMessage;

    OdenComChannelGetVarSizeMsgFunc comChannelGetVarSizeMsg;
    OdenComChannelGetLastVarSizeMsgFunc comChannelGetLastVarSizeMsg;

    void * messagingReserved[10];

    // Extended
    OdenSettingsReadSceneStringFunc readSceneString;
    OdenSettingsWriteSceneStringFunc writeSceneString;
    OdenExitApplicationFunc exitApplication;
    OdenSettingsReadStringExFunc readStringEx;
    OdenClearLinkEncryptionAllowedPublicKeysFunc clearLinkEncryptionAllowedPublicKeys;
    OdenSetLinkToRelayLinkFunc setLinkToRelayLink;
    OdenIsLinkRelayLinkFunc isLinkRelayLink;
    OdenSetLinkSendToRelayFunc setLinkSendToRelay;
    OdenIsLinkSendingToRelayFunc isLinkSendingToRelay;
    OdenSetLinkDropOutgoingPackets setLinkDropOutgoingPackets;
    OdenSettingsGetProjectFilePathFunc getProjectFilePath;
    OdenSetMouseCursorFunc setMouseCursor;
    OdenAddLinkP2pExcludedSubnetFunc addLinkP2pExcludedSubnet;
    OdenClearLinkP2pExcludedSubnetsFunc clearLinkP2pExcludedSubnets;
    OdenGetLicenseHashFunc getLicenseHash;
    OdenGetDewarpingParamsFunc getDewarpingParams;
    OdenSetDewarpingParamsFunc setDewarpingParams;
    OdenGetLinkTimeSinceLastReceivedPacketNsFunc getLinkTimeSinceLastReceivedPacketNs;

    OdenConfigureTextureStreamingFunc configureTextureStreaming;
    OdenStartTextureStreamingFunc startTextureStreaming;
    OdenStopTextureStreamingFunc stopTextureStreaming;
    OdenGetStreamerLinkFeedbackStatsFunc getStreamerLinkFeedbackStats;
    OdenGetStreamPacketLossFunc getStreamPacketLoss;
    OdenSetLinkAllowOutgoingFeedbackFunc setLinkAllowOutgoingFeedback;
    void * reserved[173];

    OdenMatrix4 worldMatrix;
    OdenMatrix4 projMatrix;
    int32_t viewportX;
    int32_t viewportY;
    int32_t viewportWidth;
    int32_t viewportHeight;
    int32_t targetWidth;
    int32_t targetHeight;

} OdenPluginEntityDrawParams;

typedef struct OdenPluginEntityGuiParams_s {
    void * entityUserData;

    // Settings
    OdenSettingsReadIntFunc readInt;
    OdenSettingsReadFloatFunc readFloat;
    OdenSettingsReadStringFunc readString;
    OdenSettingsWriteIntFunc writeInt;
    OdenSettingsWriteFloatFunc writeFloat;
    OdenSettingsWriteStringFunc writeString;
    OdenSettingsGetProjectPathFunc getProjectPath;
    OdenSettingsGetProjectPluginsPathFunc getProjectPluginsPath;
    OdenGetApplicationTypeFunc getApplicationType;
    OdenSetHasCrashedFunc setHasCrashed;
    OdenSetUiDisabledFunc setUiDisabled;
    OdenSettingsReadSceneStringFunc readSceneString;
    OdenSettingsWriteSceneStringFunc writeSceneString;
    OdenExitApplicationFunc exitApplication;
    OdenSettingsReadStringExFunc readStringEx;
    OdenSettingsGetProjectFilePathFunc getProjectFilePath;
    OdenGetOdenVersionFunc getOdenVersion;
    OdenGetLicenseHashFunc getLicenseHash;
    void * settingsReserved[4];

    // GUI
    OdenGuiLabelFunc label;
    OdenGuiButtonFunc button;
    OdenGuiInactiveButtonFunc inactiveButton;
    OdenGuiComboFunc combo;
    OdenGuiInactiveComboFunc inactiveCombo;
    OdenGuiSliderIntFunc sliderInt;
    OdenGuiSliderFloatFunc sliderFloat;
    OdenGuiSliderFloat3Func sliderFloat3;
    OdenGuiCheckboxFunc checkbox;
    OdenGuiTreeNodeFunc treeNode;
    OdenGuiTreePopFunc treePop;
    OdenGuiDragIntFunc dragInt;
    OdenGuiDragInt2Func dragInt2;
    OdenGuiDragFloatFunc dragFloat;
    OdenGuiDragFloat2Func dragFloat2;
    OdenGuiDragFloat3Func dragFloat3;
    OdenGuiDragFloat4Func dragFloat4;
    OdenGuiDragFloat5Func dragFloat5;
    OdenGuiSameLineFunc sameLine;
    OdenGuiInputText inputText;
    OdenGuiLabelColoredFunc labelColored;
    OdenGuiDragInt3Func dragInt3;
    OdenGuiDragInt4Func dragInt4;
    OdenGuiDragInt5Func dragInt5;
    OdenGuiTooltipIfLastItemHoveredFunc tooltipIfLastItemHovered;
    OdenGuiGraphFunc graph;
    OdenGuiIndentFunc indent;
    OdenGuiUnindentFunc unindent;
    OdenColorEdit3Func colorEdit3;
    OdenColorEdit4Func colorEdit4;
    void * guiReserved[245];

    // Scene
    OdenGetHorizonFunc getHorizon;
    OdenSetHorizonFunc setHorizon;
    OdenSetSeparateProjectionsFunc setSeparateProjections;
    OdenSetCurtainFunc setCurtain;
    OdenSetStitchDistanceFunc setStitchDistance;
    OdenSetCameraBrightnessFunc setCameraBrightness;
    union {
        DEPRECATED("Rename to reflect that it works for GigE cameras too")
        OdenGetAllConnectedGigECameraSerialsFunc getAllConnectedIdsCameraSerials;
        OdenGetAllConnectedGigECameraSerialsFunc getAllConnectedGigECameraSerials;
    };
    OdenSetPanoramaVideoSettingsStringFunc setPanoramaVideoSettingsString;
    OdenSetCameraCalibrationStringFunc setCameraCalibrationString;
    OdenGetCameraCalibrationFunc getCameraCalibration;
    OdenSetCameraCalibrationFunc setCameraCalibration;
    OdenGetCameraCropDataFunc getCameraCropData;
    OdenGetCameraMetadataFunc getCameraMetadata;
    OdenGetCameraTemperatureFunc getCameraTemperature;
    OdenGetCameraFpsFunc getCameraFps;
    OdenGetCameraLastValidTimestampFunc getCameraLastValidTimestamp;
    OdenSetEntityEnabledFunc setEntityEnabled;
    OdenSetEntityTransformFunc setEntityTransform;
    OdenSetVirtualCameraPositionFunc setVirtualCameraPosition;
    OdenSetVirtualCameraRotationFunc setVirtualCameraRotation;
    OdenSetVirtualCameraFovFunc setVirtualCameraFov;
    OdenSetVirtualCameraResolutionFunc setVirtualCameraResolution;
    OdenSetVirtualCameraCircularCropFunc setVirtualCameraCircularCrop;
    OdenGetFaultsFunc getFaults;
    OdenShowHmdManualAdjustmentPatternFunc showHmdManualAdjustmentPattern;
    OdenInhibitMouseLookFunc inhibitMouseLook;
    OdenPublishData publishData;
    OdenQueryData queryData;
    OdenHasEntityWithNameFunc hasEntityWithName;
    OdenGetEntityWorldMatrixFunc getEntityWorldMatrix;
    OdenGetPanoramaVideoSettingsStringFunc getPanoramaVideoSettingsString;
    OdenGetCameraRigSettingsStringFunc getCameraRigSettingsString;
    OdenGetCameraCurrentTimestampFunc getCameraCurrentTimestamp;
    OdenIsJoystickPresentFunc isJoystickPresent;
    OdenGetJoystickStateFunc getJoystickState;
    DEPRECATED("Traffic can come from several ip addresses, this function will give you the first one")
    OdenGetStreamSenderIpFunc getStreamSenderIp;
    OdenGetStreamStatisticsFunc getStreamStatistics;
    OdenGetAllSceneIdsFunc getAllSceneIds;
    OdenGetSelectedSceneIdFunc getSelectedSceneId;
    OdenSetSelectedSceneFunc setSelectedScene;
    OdenGetSceneNameFunc getSceneName;
    OdenRawRecordStartFunc rawRecordStart;
    OdenRawRecordStopFunc rawRecordStop;
    OdenRawRecordSetEntityRingBufferSizeFunc rawRecordSetEntityRingBufferSize;
    OdenRawRecordEntityRingBufferEnableFunc rawRecordEntityRingBufferEnable;
    OdenPlaybackIsPlayingFunc playbackIsPlaying;
    OdenPlaybackStartFunc playbackStart;
    OdenPlaybackPauseFunc playbackPause;
    OdenPlaybackGetTimesFunc playbackGetTimes;
    OdenPlaybackSetTimeFunc playbackSetTime;
    OdenPlaybackAdvanceTimeFunc playbackAdvanceTime;
    OdenPlaybackSetLoopFunc playbackSetLoop;
    DEPRECATED("Use the link functions instead")
    OdenGetStreamerDestinationIpFunc getStreamerDestinationIp;
    union {
        DEPRECATED("Rename to reflect that it works for GigE cameras too")
        OdenGetGigECameraSerial getIdsCameraSerial;
        OdenGetGigECameraSerial getGigECameraSerial;
    };
    OdenSetStreamerFrameMetadata setStreamerFrameMetadata;
    OdenSetCameraResolutionFunc setCameraResolution;
    OdenSetCameraGammaFunc setCameraGamma;
    OdenSetCameraAutoExposureRoiFunc setCameraAutoExposureRoi;
    OdenLoadProjectFunc loadProject;
    OdenSaveProjectFunc saveProject;
    OdenProjectHasUnsavedChangesFunc projectHasUnsavedChanges;
    OdenModelEntitySetPathFunc modelEntitySetPath;
    OdenSetStreamerTargetChannelUtilizationFunc setStreamerTargetChannelUtilization;
    OdenSetStreamerMaxBandwidthFunc setStreamerMaxBandwidth;
    OdenSetCameraStreamStateFunc setCameraStreamState;
    OdenGetCameraStreamStateFunc getCameraStreamState;
    OdenSetForceFeedbackAngleFunc setForceFeedbackAngle;
    OdenGetStreamStatisticsExFunc getStreamStatisticsEx;
    OdenGetStreamerBitrateMbpsFunc getStreamerBitrateMbps;
    OdenStartStreamerFunc startStreamer;
    OdenStopStreamerFunc stopStreamer;
    OdenIsStreamerRunningFunc isStreamerRunning;
    DEPRECATED("Use the link functions instead")
    OdenSetStreamerDestinationIpFunc setStreamerDestinationIp;
    DEPRECATED("Use the link functions instead")
    OdenSetStreamerDestinationPortFunc setStreamerDestinationPort;
    DEPRECATED("Use the link functions instead")
    OdenGetStreamerDestinationPortFunc getStreamerDestinationPort;
    DEPRECATED("Use the link functions instead")
    OdenSetStreamerAudioDestinationPortFunc setStreamerAudioDestinationPort;
    DEPRECATED("Use the link functions instead")
    OdenGetStreamerAudioDestinationPortFunc getStreamerAudioDestinationPort;
    DEPRECATED("Use the link functions instead")
    OdenSetStreamerFeedbackDestinationPortFunc setStreamerFeedbackDestinationPort;
    DEPRECATED("Use the link functions instead")
    OdenGetStreamerFeedbackDestinationPortFunc getStreamerFeedbackDestinationPort;
    DEPRECATED("Use the link functions instead")
    OdenSetComChannelDestinationPortFunc setComChannelDestinationPort;
    DEPRECATED("Use the link functions instead")
    OdenGetComChannelDestinationPortFunc getComChannelDestinationPort;
    DEPRECATED("Use the link functions instead")
    OdenSetComChannelReceivePortFunc setComChannelReceivePort;
    DEPRECATED("Use the link functions instead")
    OdenGetComChannelReceivePortFunc getComChannelReceivePort;
    DEPRECATED("Use the link functions instead")
    OdenSetVideoReceivePortFunc setVideoReceivePort;
    DEPRECATED("Use the link functions instead")
    OdenGetVideoReceivePortFunc getVideoReceivePort;
    DEPRECATED("Use the link functions instead")
    OdenSetRealtimeAudioLinkReceivePortFunc setRealtimeAudioLinkReceivePort;
    DEPRECATED("Use the link functions instead")
    OdenGetRealtimeAudioLinkReceivePortFunc getRealtimeAudioLinkReceivePort;
    DEPRECATED("Use the link functions instead")
    OdenSetRemoteStreamerFeedbackReceivePortFunc setRemoteStreamerFeedbackReceivePort;
    DEPRECATED("Use the link functions instead")
    OdenGetRemoteStreamerFeedbackReceivePortFunc getRemoteStreamerFeedbackReceivePort;
    OdenRawRecordIsRunningFunc rawRecordIsRunning;
    DEPRECATED("No-op: Producer outputs no longer exist")
    OdenStartEncodedProducerOutputsFunc startEncodedProducerOutputs;
    OdenApplyProjectTemplateFunc applyProjectTemplate;
    OdenGetLinkCountFunc getLinkCount;
    OdenSetLinkModeFunc setLinkMode;
    OdenGetLinkModeFunc getLinkMode;
    OdenSetLinkBindIpFunc setLinkBindIp;
    OdenGetLinkBindIpFunc getLinkBindIp;
    OdenSetLinkReceivePortFunc setLinkReceivePort;
    OdenGetLinkReceivePortFunc getLinkReceivePort;
    OdenSetLinkDestinationIpFunc setLinkDestinationIp;
    OdenGetLinkDestinationIpFunc getLinkDestinationIp;
    OdenSetLinkDestinationPortFunc setLinkDestinationPort;
    OdenGetLinkDestinationPortFunc getLinkDestinationPort;
    OdenGetAllEntityUuidsFunc getAllEntityUuids;
    OdenGetEntityTypeNameFunc getEntityTypeName;
    OdenGetBasicStatisticsFunc getBasicStatistics;
    OdenSetLinkEncryptionPrivateKeyPathFunc setLinkEncryptionPrivateKeyPath;
    OdenGetLinkEncryptionPrivateKeyPathFunc getLinkEncryptionPrivateKeyPath;
    OdenAddLinkEncryptionAllowedPublicKeyFunc addLinkEncryptionAllowedPublicKey;
    OdenRemoveLinkEncryptionAllowedPublicKeyFunc removeLinkEncryptionAllowedPublicKey;
    OdenGetLinkEncryptionAllowedPublicKeysCountFunc getLinkEncryptionAllowedPublicKeysCount;
    OdenGetLinkEncryptionAllowedPublicKeyFunc getLinkEncryptionAllowedPublicKey;
    DEPRECATED("No-op: Producer removed. Kept for ABI compatibility. Always returns false.")
    OdenGetLiveSceneIdFunc getLiveSceneId;
    DEPRECATED("No-op: Producer removed. Kept for ABI compatibility. Always returns false.")
    OdenSetLiveSceneFunc setLiveScene;
    OdenShowHmdControllersFunc showHmdControllers;
    OdenSetFocusRegionCenterPixelPositionFunc setFocusRegionCenterPixelPosition;
    OdenGetEntityVideoSizeFunc getEntityVideoSize;
    OdenSetRegulatorModeFunc setRegulatorMode;
    OdenGetRegulatorModeFunc getRegulatorMode;
    OdenSetForceFeedbackForceFunc setForceFeedbackForce;
    OdenSetCloneStreamSourceFunc setCloneStreamSource;
    OdenGetSceneContainingPluginEntityFunc getSceneContainingPluginEntity;
    OdenSetLinkEncryptionInternalSourceIpFunc setLinkEncryptionInternalSourceIp;
    OdenGetLinkEncryptionInternalSourceIpFunc getLinkEncryptionInternalSourceIp;
    OdenSetLinkEncryptionInternalDestinationIpFunc setLinkEncryptionInternalDestinationIp;
    OdenGetLinkEncryptionInternalDestinationIpFunc getLinkEncryptionInternalDestinationIp;
    OdenAddLinkFunc addLink;
    OdenRemoveLinkFunc removeLink;
    OdenGetInstanceUuidFunc getInstanceUuid;
    OdenSetLinkP2pEnabledFunc setLinkP2pEnabled;
    OdenGetLinkP2pEnabledFunc getLinkP2pEnabled;
    OdenGetLinkP2pStatusFunc getLinkP2pStatus;
    OdenSetLinkP2pStunServerFunc setLinkP2pStunServer;
    OdenGetLinkP2pStunServerFunc getLinkP2pStunServer;
    OdenGetCalibratedFloorHeightFunc getCalibratedFloorHeight;
    OdenGetEntityNameFunc getEntityName;
    OdenGetCameraLastReceivedFrameTimeFunc getCameraLastReceivedFrameTime;
    OdenGetAlignmentIdFunc getAlignmentId;
    OdenGetVideoPlacementFunc getVideoPlacement;
    DEPRECATED("Not supported")
    OdenGetVirtualWebcamNameFunc getVirtualWebcamName;
    OdenUpdateSetAudioReceivingEnabled setAudioReceivingEnabled;
    OdenUpdateSetAudioSendingEnabled setAudioSendingEnabled;
    OdenRawRecordGetLastPath rawRecordGetLastPath;
    OdenLoadSceneFunc loadScene;
    OdenRemoveSceneFunc removeScene;
    OdenGetLinkStatisticsFunc getLinkStatistics;
    OdenGetSceneParamFunc getSceneParam;
    OdenSetSceneParamFunc setSceneParam;
    OdenCloseProjectFunc closeProject;
    OdenAddEntityFunc addEntity;
    OdenRemoveEntityFunc removeEntity;
    OdenPollRemoteCalibrationFunc pollRemoteCalibration;
    OdenSetExternalStitchGeometryFunc setExternalStitchGeometry;
    OdenGetParentUuidFunc getParentUuid;
    OdenGetEntityUuidFunc getEntityUuid;
    OdenGetGamepadStateFunc getGamepadState;

    // Camera
    OdenSetFpsCameraFieldOfViewFunc setFpsCameraFieldOfView;
    OdenSetHmdCameraFieldOfViewFactorFunc setHmdCameraFieldOfViewFactor;
    OdenGetSavedCameraStatesFunc getSavedCameraStates;
    OdenGetSavedCameraStateFunc getSavedCameraState;
    OdenSetSavedCameraStateFunc setSavedCameraState;
    OdenLerpToSavedCameraStateFunc lerpToSavedCameraState;
    OdenGetFpsCameraStateFunc getFpsCameraState;
    OdenGetFpsCameraTypeFunc getFpsCameraType;
    OdenSetFpsCameraTypeFunc setFpsCameraType;
    OdenSetGuiVisibleFunc setGuiVisible;
    OdenPushHmdPostTransformFunc pushHmdPostTransform;
    OdenGetHmdTransformFunc getHmdTransform;

    void * cameraReserved[10];

    // Input
    OdenInputIsKeyDownFunc isKeyDown;
    OdenInputIsKeyPressedSinceLastSwapFunc isKeyPressedSinceLastSwap;
    OdenInputGetKeysPressedSinceLastSwap getKeysPressedSinceLastSwap;
    OdenInputGetWindowEventsFunc getWindowEvents;
    OdenGetGamepadStateV2Func getGamepadStateV2;

    void * inputReserved[2];

    // Messaging
    OdenComChannelMaxMessageSizeFunc comChannelMaxMessageSize;
    OdenComChannelSendMessageFunc comChannelSendMessage;
    OdenComChannelGetMessageCountFunc comChannelGetMessageCount;
    OdenComChannelGetMessageFunc comChannelGetMessage;
    OdenComChannelGetLastMessageFunc comChannelGetLastMessage;

    OdenComChannelGetVarSizeMsgFunc comChannelGetVarSizeMsg;
    OdenComChannelGetLastVarSizeMsgFunc comChannelGetLastVarSizeMsg;

    void * messagingReserved[10];

    // Extended
    OdenClearLinkEncryptionAllowedPublicKeysFunc clearLinkEncryptionAllowedPublicKeys;
    OdenSetLinkToRelayLinkFunc setLinkToRelayLink;
    OdenIsLinkRelayLinkFunc isLinkRelayLink;
    OdenSetLinkSendToRelayFunc setLinkSendToRelay;
    OdenIsLinkSendingToRelayFunc isLinkSendingToRelay;
    OdenSetLinkDropOutgoingPackets setLinkDropOutgoingPackets;
    OdenSetMouseCursorFunc setMouseCursor;
    OdenAddLinkP2pExcludedSubnetFunc addLinkP2pExcludedSubnet;
    OdenClearLinkP2pExcludedSubnetsFunc clearLinkP2pExcludedSubnets;
    OdenGetDewarpingParamsFunc getDewarpingParams;
    OdenSetDewarpingParamsFunc setDewarpingParams;
    OdenGetLinkTimeSinceLastReceivedPacketNsFunc getLinkTimeSinceLastReceivedPacketNs;

    OdenConfigureTextureStreamingFunc configureTextureStreaming;
    OdenStartTextureStreamingFunc startTextureStreaming;
    OdenStopTextureStreamingFunc stopTextureStreaming;
    OdenGetStreamerLinkFeedbackStatsFunc getStreamerLinkFeedbackStats;
    OdenGetStreamPacketLossFunc getStreamPacketLoss;
    OdenSetLinkAllowOutgoingFeedbackFunc setLinkAllowOutgoingFeedback;
    void * reserved[198];

} OdenPluginEntityGuiParams;

typedef struct OdenPluginEntityDataRecordingPlaybackParams_s {
    void * entityUserDataThreadUnsafe;
    const uint8_t * data;
    int32_t dataSize;
} OdenPluginEntityDataRecordingPlaybackParams;

typedef struct OdenPluginEntityMigrateHotreloadUserDataParams_s {
    void * entityUserData;
    uint8_t * storage;
    int32_t storageSize;
} OdenPluginEntityMigrateHotreloadUserDataParams;

typedef void (*OdenPluginEntityInitFunc)(OdenPluginEntityInitParams * api);
typedef void (*OdenPluginEntityShutdownFunc)(OdenPluginEntityShutdownParams * api);
typedef void (*OdenPluginEntityUpdateFunc)(OdenPluginEntityUpdateParams * api);
typedef void (*OdenPluginEntityDrawFunc)(OdenPluginEntityDrawParams * api);
typedef void (*OdenPluginEntityGuiFunc)(OdenPluginEntityGuiParams * api);
typedef void (*OdenPluginEntityDataRecordingPlaybackFunc)(OdenPluginEntityDataRecordingPlaybackParams * api);
typedef void (*OdenPluginEntitySaveStateForHotreloadFunc)(
    OdenPluginEntityMigrateHotreloadUserDataParams * api);
typedef void (*OdenPluginEntityLoadStateForHotreloadFunc)(
    OdenPluginEntityMigrateHotreloadUserDataParams * api);

// Registration

typedef struct OdenPluginEntityRegistration_s {
    char entityId[PLUGIN_ENTITY_ID_SIZE + 1];
    char entityName[PLUGIN_ENTITY_NAME_SIZE + 1];

    OdenPluginEntityInitFunc init;
    OdenPluginEntityShutdownFunc shutdown;
    OdenPluginEntityUpdateFunc update;
    OdenPluginEntityDrawFunc draw;
    OdenPluginEntityGuiFunc gui;
    OdenPluginEntityDataRecordingPlaybackFunc recordingPlayback;
    OdenPluginEntitySaveStateForHotreloadFunc saveStateForReload;
    OdenPluginEntityLoadStateForHotreloadFunc loadStateForReload;
    int32_t migrateSizeRequirements;

    void * reserved[16];

} OdenPluginEntityRegistration;

typedef struct OdenPluginVideoCaptureInitParams_s OdenPluginVideoCaptureInitParams;
typedef struct OdenPluginVideoCaptureShutdownParams_s OdenPluginVideoCaptureShutdownParams;
typedef struct OdenPluginVideoCaptureUpdateParams_s OdenPluginVideoCaptureUpdateParams;
typedef struct OdenPluginVideoCaptureGuiParams_s OdenPluginVideoCaptureGuiParams;

typedef void (*OdenPluginVideoCaptureInitFunc)(OdenPluginVideoCaptureInitParams * api);
typedef void (*OdenPluginVideoCaptureShutdownFunc)(OdenPluginVideoCaptureShutdownParams * api);
typedef void (*OdenPluginVideoCaptureUpdateFunc)(OdenPluginVideoCaptureUpdateParams * api);
typedef void (*OdenPluginVideoCaptureGuiFunc)(OdenPluginVideoCaptureGuiParams * api);

typedef struct OdenPluginVideoCaptureRegistration_s {
    char videoCaptureId[PLUGIN_VIDEO_CAPTURE_ID_SIZE + 1];
    char videoCaptureName[PLUGIN_VIDEO_CAPTURE_NAME_SIZE + 1];

    OdenPluginVideoCaptureInitFunc init;
    OdenPluginVideoCaptureShutdownFunc shutdown;
    OdenPluginVideoCaptureUpdateFunc update;
    OdenPluginVideoCaptureGuiFunc gui;

    void * reserved[32];

} OdenPluginVideoCaptureRegistration;

typedef void (*OdenPluginRegisterEntityFunc)(OdenPluginEntityRegistration * entityRegistration);
typedef void (*OdenPluginInstantiateGlobalEntityFunc)(OdenPluginEntityRegistration * entityRegistration);
typedef void (*OdenPluginRegisterVideoCaptureFunc)(
    OdenPluginVideoCaptureRegistration * videoCaptureRegistration);

// Plugin registration

typedef struct OdenPluginRegistrationParams_s {
    OdenPluginRegisterEntityFunc registerEntity;
    OdenPluginInstantiateGlobalEntityFunc instantiateGlobalEntity;
    OdenPluginRegisterVideoCaptureFunc registerVideoCapture;

    void * reserved[13];

} OdenPluginRegistrationParams;

typedef void (*OdenPluginRegisterFunc)(
    OdenPluginRegistrationParams * p, const OdenPluginGlobalFunctions * global);

#ifdef __cplusplus
}
#endif

#define ODEN_PLUGIN_VERSION 13

#ifdef _MSC_VER
#ifdef __cplusplus
extern "C" {
#endif
__declspec(dllexport) int32_t oden_plugin_version(void);
__declspec(dllexport) void oden_plugin_register(
    OdenPluginRegistrationParams * p, const OdenPluginGlobalFunctions * global);
#ifdef __cplusplus
}
#endif

// clang-format off
#define ODEN_PLUGIN(...)                                                                                     \
    const OdenPluginGlobalFunctions * oden;                                                                  \
    extern "C" __declspec(dllexport) int32_t oden_plugin_version(void) {                                     \
        return ODEN_PLUGIN_VERSION;                                                                          \
    }                                                                                                        \
    static void oden_plugin_register_inner(OdenPluginRegistrationParams * p);                                \
    extern "C" __declspec(dllexport) void oden_plugin_register(                                              \
        OdenPluginRegistrationParams * p, const OdenPluginGlobalFunctions * global) {                        \
        oden = global;                                                                                       \
        oden_plugin_register_inner(p);                                                                       \
    }                                                                                                        \
    static void oden_plugin_register_inner(__VA_ARGS__)
#else
#define ODEN_PLUGIN(...)                                                                                     \
    const OdenPluginGlobalFunctions * oden;                                                                  \
    extern "C" __attribute__((visibility("default"))) int32_t oden_plugin_version(void);                     \
    extern "C" __attribute__((visibility("default"))) int32_t oden_plugin_version(void) {                    \
        return ODEN_PLUGIN_VERSION;                                                                          \
    }                                                                                                        \
    static void oden_plugin_register_inner(OdenPluginRegistrationParams * p);                                \
    extern "C" __attribute__((visibility("default"))) void oden_plugin_register(                             \
        OdenPluginRegistrationParams * p, const OdenPluginGlobalFunctions * global);                         \
    extern "C" __attribute__((visibility("default"))) void oden_plugin_register(                             \
        OdenPluginRegistrationParams * p, const OdenPluginGlobalFunctions * global) {                        \
        oden = global;                                                                                       \
        oden_plugin_register_inner(p);                                                                       \
    }                                                                                                        \
    static void oden_plugin_register_inner(__VA_ARGS__)
#endif
// clang-format on
