-module('10').

-export([main/1]).

-define(report(N1, N2), N1 =:= 17 andalso N2 =:= 61).

-record(bot, {nlow, nhigh, action, report}).

main(_) ->
    Ps = #{},
    try
        read_loop(Ps)
    catch X:Y ->
              io:format("~p:~p occured, ~p~n", [X, Y, erlang:get_stacktrace()])
    end.

read_loop(Ps) ->
    case io:get_line([]) of
        "value "++Str ->
            [NStr, BotStr|_] = re:split(Str, " goes to bot |\n"),
            N = binary_to_integer(NStr),
            {BotPid, Ps1} = get_bot_or_output(BotStr, Ps),
            BotPid ! {n, N},
            read_loop(Ps1);
        "bot "++Str ->
            [BotStr, LBotStr, HBotStr|_] =
                re:split(Str, " gives low to | and high to |\n"),
            {BotPid, Ps1} = get_bot_or_output(BotStr, Ps),
            {LBotPid, Ps2} = get_bot_or_output(LBotStr, Ps1),
            {HBotPid, Ps3} = get_bot_or_output(HBotStr, Ps2),
            BotPid ! {action, fun(B = #bot{nlow = N1, nhigh = N2})
                                    when N1 =:= undefined; N2 =:= undefined ->
                                      B;
                                 (B = #bot{nlow = N1, nhigh = N2}) ->
                                      LBotPid ! {n, N1},
                                      HBotPid ! {n, N2},
                                      B#bot{nlow = undefined,
                                           nhigh = undefined}
                              end},
            read_loop(Ps3);
        _ ->
            Me = self(),
            Collect = maps:fold(fun({output, N}, Pid, Acc) ->
                                        Pid ! {quit, Me, N},
                                        Acc + 1;
                                   (_, _, Acc) ->
                                        Acc
                                end, 0, Ps),
            collect_loop(Collect+1, Ps)
    end.

collect_loop(0, _) ->
    ok;
collect_loop(N, Ps) ->
    receive
        {report, Pid} ->
            {bot, BotId} = maps:fold(fun(K, V, _) when V =:= Pid ->
                                             K;
                                        (_, _, Acc) ->
                                             Acc
                                     end, undefined, Ps),
            io:format("Bot ~p was responsible for comparing~n", [BotId]),
            collect_loop(N-1, Ps);
        {report_output, M, Chips} ->
            io:format("Output ~p contains ~p~n", [M, Chips]),
            collect_loop(N-1, Ps)
    after 5000 ->
              io:format("timeout~n")
    end.

get_bot_or_output(<<"bot ",N/binary>>, Ps) ->
    get_bot(binary_to_integer(N), Ps);
get_bot_or_output(<<"output ",N/binary>>, Ps) ->
    get_output(binary_to_integer(N), Ps);
get_bot_or_output(N, Ps) ->
    get_bot(binary_to_integer(N), Ps).

get_bot(N, Ps) ->
    case maps:find({bot, N}, Ps) of
        {ok, Pid} -> {Pid, Ps};
        error -> get_bot(N, spawn_bot(N, Ps))
    end.

get_output(N, Ps) ->
    case maps:find({output, N}, Ps) of
        {ok, Pid} -> {Pid, Ps};
        error ->
            Pid = spawn_link(fun() -> output_loop([]) end),
            {Pid, Ps#{{output, N} => Pid}}
    end.

spawn_bot(Num, Ps) ->
    Me = self(),
    NewBot = #bot{
                action = fun(X) -> X end,
                report = fun(#bot{nlow = N1, nhigh = N2}) ->
                                 if ?report(N1, N2) -> Me ! {report, self()};
                                     true -> ok
                                 end
                         end
               },
    Ps#{{bot, Num} => spawn_link(fun() -> bot_loop(NewBot) end)}.

bot_loop(Bot0) ->
    Bot =
        receive
            {n, N} ->
                Low = Bot0#bot.nlow,
                if Low =:= undefined ->
                       Bot0#bot{nlow = N};
                   N < Low ->
                       Bot0#bot{nlow = N, nhigh = Low};
                   true ->
                       Bot0#bot{nhigh = N}
                end;
            {action, Fun} ->
                Bot0#bot{action = Fun}
        end,
    (Bot#bot.report)(Bot),
    NewBot = (Bot#bot.action)(Bot),
    bot_loop(NewBot).

output_loop(L) ->
    receive
        {n, N} ->
            output_loop([N|L]);
        {quit, Master, N} ->
            Master ! {report_output, N, L}
    end.
