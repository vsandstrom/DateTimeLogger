#!/bin/sh
CMD='select name, date, time from data join users on data.user_id = users.id order by data.date desc, data.time desc limit 1;'
echo "$CMD" | sqlite3 ./db.db
