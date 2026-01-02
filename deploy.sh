set -e

echo "Starting deployment..."
cd /home/mediascraper/mediascraper
echo "Pulling latest code..."
git pull origin main
echo "Building..."
cargo build --release
echo "Restarting service..."
sudo systemctl restart mediascraper
echo "Deployment complete!"