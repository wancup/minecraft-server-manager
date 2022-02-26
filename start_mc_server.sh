#!/bin/sh

minecraft_path=["your-minecraft-path"]

# backup
# If you want to upload world data to s3, uncomment next block
# s3_bucket_name=["your-backup-bucket-name"]
# backup_file_name="$minecraft_path"/backup/mc_world_$(date "+%Y%m%d").tar.gz
# tar -zcvf "$backup_file_name" "$minecraft_path"/world
# aws s3 cp "$backup_file_name" s3://"$s3_bucket_name"/"$(date '+%Y%m')"/
# rm -f "$backup_file_name"

# start sever
java -Xms1024M -Xmx2048M -jar "$minecraft_path"/minecraft_server.xxx.jar nogui

