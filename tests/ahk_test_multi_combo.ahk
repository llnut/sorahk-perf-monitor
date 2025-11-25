; AutoHotkey Performance Test Script
; Test Case: Multiple Combo Keys with Shared Modifier
; Trigger: LAlt+1->A, LAlt+2->B, LAlt+3->C

#NoEnv
#SingleInstance Force
SetBatchLines, -1
SetKeyDelay, -1, -1

; Configuration
global turboInterval := 5
global eventDuration := 2

; State for each combo
global pressedA := false
global pressedB := false
global pressedC := false
global timerA := false
global timerB := false
global timerC := false

; LAlt + 1 -> A
LAlt & 1::
    if (!pressedA) {
        pressedA := true
        SendInput, {a down}
        Sleep, %eventDuration%
        SendInput, {a up}
        
        if (!timerA) {
            timerA := true
            SetTimer, TurboFireA, %turboInterval%
        }
    }
    return

; LAlt + 2 -> B
LAlt & 2::
    if (!pressedB) {
        pressedB := true
        SendInput, {b down}
        Sleep, %eventDuration%
        SendInput, {b up}
        
        if (!timerB) {
            timerB := true
            SetTimer, TurboFireB, %turboInterval%
        }
    }
    return

; LAlt + 3 -> C
LAlt & 3::
    if (!pressedC) {
        pressedC := true
        SendInput, {c down}
        Sleep, %eventDuration%
        SendInput, {c up}
        
        if (!timerC) {
            timerC := true
            SetTimer, TurboFireC, %turboInterval%
        }
    }
    return

; Release handling
#If (pressedA || pressedB || pressedC)
LAlt up::
1 up::
2 up::
3 up::
    if (pressedA) {
        pressedA := false
        timerA := false
        SetTimer, TurboFireA, Off
    }
    if (pressedB) {
        pressedB := false
        timerB := false
        SetTimer, TurboFireB, Off
    }
    if (pressedC) {
        pressedC := false
        timerC := false
        SetTimer, TurboFireC, Off
    }
    return
#If

TurboFireA:
    if (pressedA) {
        SendInput, {a down}
        Sleep, %eventDuration%
        SendInput, {a up}
    }
    return

TurboFireB:
    if (pressedB) {
        SendInput, {b down}
        Sleep, %eventDuration%
        SendInput, {b up}
    }
    return

TurboFireC:
    if (pressedC) {
        SendInput, {c down}
        Sleep, %eventDuration%
        SendInput, {c up}
    }
    return

Esc::ExitApp

