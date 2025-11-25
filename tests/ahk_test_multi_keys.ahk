; AutoHotkey Performance Test Script
; Test Case: Multiple Keys Turbo (3 independent turbo keys)
; Trigger: A->1, B->2, C->3

#NoEnv
#SingleInstance Force
SetBatchLines, -1
SetKeyDelay, -1, -1

; Configuration
global turboInterval := 5
global eventDuration := 2

; State tracking for each key
global pressedA := false
global pressedB := false
global pressedC := false
global timerA := false
global timerB := false
global timerC := false

; Key A -> 1
$a::
    if (!pressedA) {
        pressedA := true
        SendInput, {1 down}
        Sleep, %eventDuration%
        SendInput, {1 up}
        
        if (!timerA) {
            timerA := true
            SetTimer, TurboFireA, %turboInterval%
        }
    }
    return

$a up::
    pressedA := false
    timerA := false
    SetTimer, TurboFireA, Off
    return

TurboFireA:
    if (pressedA) {
        SendInput, {1 down}
        Sleep, %eventDuration%
        SendInput, {1 up}
    }
    return

; Key B -> 2
$b::
    if (!pressedB) {
        pressedB := true
        SendInput, {2 down}
        Sleep, %eventDuration%
        SendInput, {2 up}
        
        if (!timerB) {
            timerB := true
            SetTimer, TurboFireB, %turboInterval%
        }
    }
    return

$b up::
    pressedB := false
    timerB := false
    SetTimer, TurboFireB, Off
    return

TurboFireB:
    if (pressedB) {
        SendInput, {2 down}
        Sleep, %eventDuration%
        SendInput, {2 up}
    }
    return

; Key C -> 3
$c::
    if (!pressedC) {
        pressedC := true
        SendInput, {3 down}
        Sleep, %eventDuration%
        SendInput, {3 up}
        
        if (!timerC) {
            timerC := true
            SetTimer, TurboFireC, %turboInterval%
        }
    }
    return

$c up::
    pressedC := false
    timerC := false
    SetTimer, TurboFireC, Off
    return

TurboFireC:
    if (pressedC) {
        SendInput, {3 down}
        Sleep, %eventDuration%
        SendInput, {3 up}
    }
    return

Esc::ExitApp

