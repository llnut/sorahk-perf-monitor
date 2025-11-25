; AutoHotkey Performance Test Script
; Test Case: Combo Key Turbo
; Trigger: LAlt+1 -> Target: A (auto-repeat)

#NoEnv
#SingleInstance Force
SetBatchLines, -1
SetKeyDelay, -1, -1

; Configuration
global turboInterval := 5
global eventDuration := 2
global isPressed := false
global timerActive := false

; Combo key: LAlt + 1 -> A
LAlt & 1::
    if (!isPressed) {
        isPressed := true
        ; Suppress the Alt modifier when sending A
        SendInput, {a down}
        Sleep, %eventDuration%
        SendInput, {a up}
        
        if (!timerActive) {
            timerActive := true
            SetTimer, TurboFire, %turboInterval%
        }
    }
    return

; Release handling - need to check both Alt and 1
#If (isPressed)
LAlt up::
1 up::
    isPressed := false
    timerActive := false
    SetTimer, TurboFire, Off
    return
#If

TurboFire:
    if (isPressed) {
        SendInput, {a down}
        Sleep, %eventDuration%
        SendInput, {a up}
    }
    return

Esc::ExitApp

