This project reads osu memory over tosu and enables shooting osc strings over UDP. 
-------------------------
! This is not perfomance and compatibility checked !
-------------------------

How to use:
1. install tosu in the working folder /osu-osc/
2. start tosu
3. start the rust bin with cargo
4. open overlay.html

How it works:
tosu WebSocket
        ↓
show_interface.rs reads raw osu data
        ↓
show_logic.rs decides what to send
        ↓
show_interface.rs shoots osc on your required address

This project was meant for Show Control at events. 
I started working on the idea with ChatGPT, since I'm a novice at Programming.
I guess this can be used for many use cases, that don't want to go an online route since this is only running locally.
Next steps I had in mind are implementing an html server locally for communication with different show software or especially obs.

I'm will be working on this if I have time at the end of the year.
