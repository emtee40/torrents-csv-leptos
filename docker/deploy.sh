sudo docker build .. -f Dockerfile \
  -t torrents-csv-leptos:latest \
  -t dessalines/torrents-csv-leptos:latest
sudo docker push dessalines/torrents-csv-leptos:latest

curl -d "Torrents-csv-leptos deploy completed." ntfy.sh/dessalines
