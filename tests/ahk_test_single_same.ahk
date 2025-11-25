; AutoHotkey Performance Test Script
; Test Case: Single Key Turbo (Same Key)
; Trigger: A, Target: A (auto-repeat A)

#NoEnv
#SingleInstance Force
SetBatchLines, -1
SetKeyDelay, -1, -1

; Configuration
global turboInterval := 5      ; 5ms interval (matching sorahk)
global eventDuration := 2      ; 2ms key press duration
global isPressed := false
global timerActive := false

; Trigger key A
$a::
    if (!isPressed) {
        isPressed := true
        SendInput, {a down}
        Sleep, %eventDuration%
        SendInput, {a up}
        
        ; Start turbo
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
        SendInput, {a down}
        Sleep, %eventDuration%
        SendInput, {a up}
    }
    return

; ESC to exit
Esc::ExitApp

