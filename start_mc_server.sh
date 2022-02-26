#!/bin/sh

# backup
# If you want to upload world data to s3, uncomment next block
# s3_bucket_name=["your-backup-bucket-name"]
# backup_file_name=/home/ec2-user/backup/mc_world_$(date "+%Y%m%d").tar.gz
# tar -zcvf "$backup_file_name" /home/ec2-user/world
# aws s3 cp "$backup_file_name" s3://"$s3_bucket_name"/"$(date '+%Y%m')"/
# rm -f "$backup_file_name"

# start sever
java -Xms1024M -Xmx2048M -jar /home/ec2-user/minecraft_server.xxx.jar nogui

