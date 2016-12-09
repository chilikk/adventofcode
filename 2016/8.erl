-module('8').
-export([main/1]).

-define(XN, 50).
-define(YN, 6).

main(_) ->
    Ps = [ spawn_link(fun() -> loop({X,Y,off}) end) ||
           X <- lists:seq(0,?XN-1), Y <- lists:seq(0,?YN-1) ],
    read_input(Ps).

loop(exit) ->
    ok;
loop(S) ->
    receive
        Fun -> loop(Fun(S))
    end.

read_input(Ps) ->
    case io:get_line([]) of
        "rect "++S ->
            [X0,Y0|_] = re:split(S,"x|\n"),
            [XN, YN] = lists:map(fun erlang:binary_to_integer/1, [X0, Y0]),
            Msg = fun({X,Y,_}) when X<XN, Y<YN -> {X, Y, on};
                     (State)                  -> State
                  end,
            lists:foreach(fun(P) -> P ! Msg end, Ps),
            read_input(Ps);
        "rotate row y="++S ->
            [YN0,N0|_] = re:split(S," by |\n"),
            [YN, N] = lists:map(fun erlang:binary_to_integer/1, [YN0, N0]),
            Msg = fun({X,Y,V}) when Y =:= YN -> {(X+N) rem ?XN, Y, V};
                     (State)                 -> State
                  end,
            lists:foreach(fun(P) -> P ! Msg end, Ps),
            read_input(Ps);
        "rotate column x="++S ->
            [XN0,N0|_] = re:split(S," by |\n"),
            [XN, N] = lists:map(fun erlang:binary_to_integer/1, [XN0, N0]),
            Msg = fun({X,Y,V}) when X =:= XN -> {X, (Y+N) rem ?YN, V};
                     (State)                 -> State
                  end,
            lists:foreach(fun(P) -> P ! Msg end, Ps),
            read_input(Ps);
        _Msg ->
            Wx = wx:new(),
            Frame = wxFrame:new(Wx, -1, "Screen", [{size, {500, 90}}]),
            OnPaint = fun(_Evt, _Obj) ->
                              Paint = wxPaintDC:new(Frame),
                              Me = self(),
                              Msg = fun(St) -> Me ! St end,
                              lists:foreach(fun(P) -> P ! Msg end, Ps),
                              collect(Ps, Paint),
                              wxPaintDC:destroy(Paint)
                      end,
            wxFrame:connect(Frame, paint, [{callback, OnPaint}]),
            wxFrame:center(Frame),
            wxFrame:show(Frame),
            timer:sleep(100000)
    end.

collect(Ps, Paint) ->
    collect(Ps, Paint, 0, length(Ps)).

collect(_, _, Num, 0) ->
    io:format("Result: ~B~n", [Num]);
collect(Ps, Paint, Num, All) ->
    receive
        {X,Y,V} = _Msg ->
            if V =:= on ->
                   wxDC:drawRectangle(Paint, {10*X,10*Y,10,10});
               true -> ok
            end,
            collect(Ps, Paint, Num + if V =:= on -> 1;
                                        true -> 0
                                     end, All-1);
        Msg -> io:format("unexpected msg ~p~n",[Msg])
    end.
