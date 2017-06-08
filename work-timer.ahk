;Process ini
    IniRead, ColorAlert, work-timer.ini, Data, ColorAlert, 1
    IniRead, OnColor, work-timer.ini, Data, OnColor, B0FFFF
    IniRead, OffColor, work-timer.ini, Data, OffColor, F07070
    IniRead, IdleTime, work-timer.ini, Data, Timeout, 10
    IniRead, Program1, work-timer.ini, Data, Program1, %A_Space%
    IniRead, Program2, work-timer.ini, Data, Program2, %A_Space%
    IniRead, Program3, work-timer.ini, Data, Program3, %A_Space%
    IniRead, Program4, work-timer.ini, Data, Program4, %A_Space%
    IniRead, Program5, work-timer.ini, Data, Program5, %A_Space%
    IniRead, Program6, work-timer.ini, Data, Program6, %A_Space%
    IniRead, Program7, work-timer.ini, Data, Program7, %A_Space%
    IniRead, Program8, work-timer.ini, Data, Program8, %A_Space%
    IniRead, Program9, work-timer.ini, Data, Program9, %A_Space%
    IniRead, MoreSlots, work-timer.ini, Data, MoreSlots, 0
    IniRead, LastTime, work-timer.ini, Data, LastTime, 00:00:00
    IniRead, xSaved, work-timer.ini, Data, xSaved, 0
    IniRead, ySaved, work-timer.ini, Data, ySaved, 0
    IniWrite, %ColorAlert%, work-timer.ini, Data, ColorAlert
    IniWrite, %OnColor%, work-timer.ini, Data, OnColor
    IniWrite, %OffColor%, work-timer.ini, Data, OffColor
    IniWrite, %IdleTime%, work-timer.ini, Data, Timeout
    IniWrite, %Program1%, work-timer.ini, Data, Program1
    IniWrite, %Program2%, work-timer.ini, Data, Program2
    IniWrite, %Program3%, work-timer.ini, Data, Program3
    IniWrite, %Program4%, work-timer.ini, Data, Program4
    IniWrite, %Program5%, work-timer.ini, Data, Program5
    IniWrite, %Program6%, work-timer.ini, Data, Program6
    IniWrite, %Program7%, work-timer.ini, Data, Program7
    IniWrite, %Program8%, work-timer.ini, Data, Program8
    IniWrite, %Program9%, work-timer.ini, Data, Program9
    IniWrite, %MoreSlots%, work-timer.ini, Data, MoreSlots
    IniWrite, %LastTime%, work-timer.ini, Data, LastTime
    IniWrite, %xSaved%, work-timer.ini, Data, xSaved
    IniWrite, %ySaved%, work-timer.ini, Data, ySaved

;Move window by dragging anywhere on the GUI
    ;When left mouse down (WM_LBUTTONDOWN) is detected, call MoveGUI function
    OnMessage(0x201, "MoveGUI")
    ;Simulate dragging title bar (WM_NCLBUTTONDOWN) on active window
    MoveGUI()
    {
        PostMessage, 0xA1, 2,,, A
    }

;Create main GUI
    ;Script runs at maximum speed
    SetBatchLines, -1
    ;Set GUI qualities
    Gui -Caption +LastFound +AlwaysOnTop
    ;Set color
    Gui, Color, %OffColor%
    if(ColorAlert = 0)
        Gui, Color, %OnColor%
    ;Set display font (20pt = 27px)
    Gui, Font, S20 CDefault Bold, Courier New
    ;Create display with placeholder timer text
    Gui, Add, Text, x8 y5 vDisplayText, 00:00:00
    ;Set floats to be two digits max, no decimals
    SetFormat, Float, 02.0
    ;Reset timer
    h := m := s := "00" 
    ;Update every second
    SetTimer, Update, 1000
    ;Update now
    Gosub, Update
    ;Show GUI
    Gui, Show, w145 h37, Work Timer

;End of startup process
    Return

;Handles right click context menu
GuiContextMenu:
    ;Set color
    Gui, Color, %OffColor%
    if(ColorAlert = 0)
        Gui, Color, %OnColor%
    ;Clear any previous context menu
    Menu, contextMenu, Add
    Menu, contextMenu, DeleteAll
    ;Create context menu
    if(H = 99 && M = 59 && S = 59){
    }
    Menu, contextMenu, Add, Resume previous time, Prev
    Menu, contextMenu, Add, Go to saved position, GoToPos
    Menu, contextMenu, Add
    programDisplay := processProgram(Program1, 30)
    Menu, contextMenu, Add, Program 1`: %programDisplay%, SetProgram1
    programDisplay := processProgram(Program2, 30)
    Menu, contextMenu, Add, Program 2`: %programDisplay%, SetProgram2
    programDisplay := processProgram(Program3, 30)
    Menu, contextMenu, Add, Program 3`: %programDisplay%, SetProgram3
    if(MoreSlots = 1){
        programDisplay := processProgram(Program4, 30)
        Menu, contextMenu, Add, Program 4`: %programDisplay%, SetProgram4
        programDisplay := processProgram(Program5, 30)
        Menu, contextMenu, Add, Program 5`: %programDisplay%, SetProgram5
        programDisplay := processProgram(Program6, 30)
        Menu, contextMenu, Add, Program 6`: %programDisplay%, SetProgram6
        programDisplay := processProgram(Program7, 30)
        Menu, contextMenu, Add, Program 7`: %programDisplay%, SetProgram7
        programDisplay := processProgram(Program8, 30)
        Menu, contextMenu, Add, Program 8`: %programDisplay%, SetProgram8
        programDisplay := processProgram(Program9, 30)
        Menu, contextMenu, Add, Program 9`: %programDisplay%, SetProgram9
    }
    Menu, contextMenu, Add
    Menu, contextMenu, Add, Save current position, SavePos
    Menu, contextMenu, Add, Timeout`: %IdleTime%,  SetTimeout
    Menu, contextMenu, Add
    Menu, contextMenu, Add, Color Alert, ChangeColor
    if(ColorAlert = 1)
        Menu, contextMenu, Check, Color Alert
    Menu, contextMenu, Add, More Program Slots, ChangeSlots
    if(MoreSlots = 1)
        Menu, contextMenu, Check, More Program Slots
    Menu, contextMenu, Add
    Menu, contextMenu, Add, Reset Timer, ResetTimer
    Menu, contextMenu, Show
    Return

;Function helps create menu items that display process paths
    processProgram(program, len){
        if(StrLen(program) >= len){
            removeLen := StrLen(program) - len
            StringTrimLeft, program, program, removeLen
            Return "... " . program
        }
        if(program = "")
            Return "(Not set)"
        MsgBox, %program%
        Return program
    }

GoToPos:
    WinMove, A, , xSaved, ySaved
    Return

SavePos:
    WinGetPos, xSaved, ySaved, wMain, hMain, A
    IniWrite, %xSaved%, work-timer.ini, Data, xSaved
    IniWrite, %ySaved%, work-timer.ini, Data, ySaved
    Return

;Reset timer if maxed or otherwise
ResetTimer:
    H := 00
    M := 00
    S := 00
    GuiControl,, DisplayText, %H%:%M%:%S%
    SetTimer, Update, On
    Return

;Toggle color alert
ChangeColor:
    if(ColorAlert = 1){
        Menu, contextMenu, UnCheck, Color Alert
        ColorAlert = 0
        IniWrite, 0, work-timer.ini, Data, ColorAlert
        Gui, Color, %OnColor%
    }else{
        Menu, contextMenu, Check, Color Alert
        ColorAlert = 1
        IniWrite, 1, work-timer.ini, Data, ColorAlert
        Gui, Color, %OffColor%
    }
    Return

;Toggle more program slots
ChangeSlots:
    if(MoreSlots = 1){
        Menu, contextMenu, UnCheck, More Program Slots
        MoreSlots := 0
        IniWrite, 0, work-timer.ini, Data, MoreSlots
    }else{
        Menu, contextMenu, Check, More Program Slots
        MoreSlots := 1
        IniWrite, 1, work-timer.ini, Data, MoreSlots
    }
    Return

;Restore previous time
Prev:
    StringSplit, LastTimeA, LastTime, `:
    H := LastTimeA1
    M := LastTimeA2
    S := LastTimeA3
    GuiControl,, DisplayText, %H%:%M%:%S%
    Return

;Set allowed program
SetProgram1:
    Gosub, WaitProgram
    Program1 := activeProcess
    IniWrite, %activeProcess%, work-timer.ini, Data, Program1
    Return
SetProgram2:
    Gosub, WaitProgram
    Program2 := activeProcess
    IniWrite, %activeProcess%, work-timer.ini, Data, Program2
    Return
SetProgram3:
    Gosub, WaitProgram
    Program3 := activeProcess
    IniWrite, %activeProcess%, work-timer.ini, Data, Program3
    Return
SetProgram4:
    Gosub, WaitProgram
    Program4 := activeProcess
    IniWrite, %activeProcess%, work-timer.ini, Data, Program4
    Return
SetProgram5:
    Gosub, WaitProgram
    Program5 := activeProcess
    IniWrite, %activeProcess%, work-timer.ini, Data, Program5
    Return
SetProgram6:
    Gosub, WaitProgram
    Program6 := activeProcess
    IniWrite, %activeProcess%, work-timer.ini, Data, Program6
    Return
SetProgram7:
    Gosub, WaitProgram
    Program7 := activeProcess
    IniWrite, %activeProcess%, work-timer.ini, Data, Program7
    Return
SetProgram8:
    Gosub, WaitProgram
    Program8 := activeProcess
    IniWrite, %activeProcess%, work-timer.ini, Data, Program8
    Return
SetProgram9:
    Gosub, WaitProgram
    Program9 := activeProcess
    IniWrite, %activeProcess%, work-timer.ini, Data, Program9
    Return

;Wait for program to change
WaitProgram:
    ;Display waiting
    GuiControl,, DisplayText, Waiting...
    ;Infinite loop until window focus leaves GUI
    while (activeClass = "AutoHotkeyGUI"){
    }
    ;Wait in case of window focus quickly switching
    Sleep, 250
    ;Display timer
    GuiControl,, DisplayText, %H%:%M%:%S%
    Return

;Timeout input GUI handling
SetTimeout:
    WinGetPos, xMain, yMain, wMain, hMain, A
    Gui, Timeout: Destroy
    Gui, Timeout: -Caption +LastFound +AlwaysOnTop
    Gui, Timeout: Add, Edit, r1 limit3 vNewTimeout, [Enter] idle timeout in seconds
    Gui, Timeout: Add, Button, x-10 y-10 w1 h1 Default gTimeoutSubmit
    Gui, Timeout: Show
    WinGetPos, , , wTime, hTime, A
    WinMove, xMain + wMain / 2 - wTime / 2, yMain + hMain / 2 - hTime / 2
    Return
TimeoutGuiClose:
TimeoutGuiEscape:
    Gui, Timeout: Cancel
    Return
TimeoutSubmit:
    Gui, Timeout: Submit
    if NewTimeout is integer
        IdleTime := NewTimeout
    if NewTimeout is float
        IdleTime := Round(NewTimeout)
    if(NewTimeout < 0)
        IdleTime := 0
    IniWrite, %IdleTime%, work-timer.ini, Data, Timeout
    Return

;Handles constant updates
Update: 
    ;Stop timer if maxed
    if(H = 99 && M = 59 && S = 59){
        SetTimer, Update, Off
    }
    ;Get active window ID, class, and process path
    activeID := WinExist("A")
    WinGetClass, activeClass, ahk_id %activeID%
    WinGet, activeProcess, ProcessPath, ahk_id %activeID%
    ;Ignore explorer.exe
    if(activeProcess = "C:\Windows\explorer.exe"){
        activeID := ""
        activeClass := ""
        activeProcess := ""
    }
    ;Check if active program is non-blank and approved  
    ProgramActive = 1
    if(activeProcess = ""){
        ProgramActive := 0
    }else{
        if(activeProcess != Program1)
        if(activeProcess != Program2)
        if(activeProcess != Program3)
        if(MoreSlots = 1){
            if(activeProcess != Program4)
            if(activeProcess != Program5)
            if(activeProcess != Program6)
            if(activeProcess != Program7)
            if(activeProcess != Program8)
            if(activeProcess != Program9)
            ProgramActive := 0
        }else{
            ProgramActive := 0
        }
    }
    ;Handle idle or forbidden program
    if((A_TimeIdle > IdleTime*1000) || (ProgramActive = 0)){
        if(ColorAlert = 1)
            Gui, Color, %OffColor%
        Return
    }
    ;Handle active and allowed program
    if(ColorAlert = 1)
        Gui, Color, %OnColor%
    S += 1.0
    if(S >= 60){
        M += 1.0
        S := 00
        }
    if(M >= 60){
        H += 1.0
        M := 00
    }
    GuiControl,, DisplayText, %H%:%M%:%S%
    Return
    
GuiClose:
GuiEscape:
    IniWrite, %H%:%M%:%S%, work-timer.ini, Data, LastTime
    Exitapp