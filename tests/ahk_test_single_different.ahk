; AutoHotkey Performance Test Script
; Test Case: Single Key Turbo (Different Keys)
; Trigger: A, Target: B (press A to auto-repeat B)

#NoEnv
#SingleInstance Force
SetBatchLines, -1
SetKeyDelay, -1, -1

; Configuration
global turboInterval := 5
global eventDuration := 2
global isPressed := false
global timerActive := false

; Trigger key A -> Target key B
$a::
    if (!isPressed) {
        isPressed := true
        SendInput, {b down}
        Sleep, %eventDuration%
        SendInput, {b up}
        
        if (!timerActive) {
            timerActive := true
            SetTimer, TurboFire, %turboInterval%
        }
    }
    return

$a up::
    isPressed := false
    timerActive := false
    SetTimer, TurboFire, Off
    return

TurboFire:
    if (isPressed) {
        SendInput, {b down}
        Sleep, %eventDuration%
        SendInput, {b up}
    }
    return

Esc::ExitApp

