lua_run require("voice_optimization")
lua_run minV = Vector(1325.731934,-378.481018,163.569443) maxV = Vector(541.029175,-692.625671,-188.601532)
lua_run PrintTable(players_in_box(minV, maxV))
lua_run print(minV, maxV)
lua_run print(Vector(Entity(1):GetPos()):WithinAABox(minV, maxV))

Vector(Entity(1):GetPos())

lua_run print_all_players()

lua_run hook.Add("player_connect", "PrintTable", PrintTable)

lua_run hook.Add("PlayerCanHearPlayersVoice", "Person", function(listener, talker) print(SysTime(), talker, listener) end)
