# 🚀 NeuroLend Deployment Guide

Your NeuroLend indexer and API are ready for deployment! Here are your hosting options:

## 📊 **Current Status**

- ✅ **Indexer**: Running and found events (including yours!)
- ✅ **API Server**: Running on http://localhost:3001
- ✅ **Event Detection**: Found 1 event in block range 6938309-6939308

## 🏗️ **Architecture**

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   0G Network    │───▶│     Indexer     │───▶│   API Server    │
│  (Blockchain)   │    │  (Event Fetch)  │    │ (REST Endpoints)│
└─────────────────┘    └─────────────────┘    └─────────────────┘
                                │                       │
                                ▼                       ▼
                        ┌─────────────────┐    ┌─────────────────┐
                        │  Event Storage  │    │   Your Frontend │
                        │   (JSON Files)  │    │   (React/Vue)   │
                        └─────────────────┘    └─────────────────┘
```

## 🌐 **Frontend API URLs**

Your frontend can use these endpoints:

```javascript
const API_BASE = "http://your-domain.com/api"; // or http://localhost:3001

// Get all events
const events = await fetch(`${API_BASE}/events`).then((r) => r.json());

// Get loans
const loans = await fetch(`${API_BASE}/loans`).then((r) => r.json());

// Get statistics
const stats = await fetch(`${API_BASE}/stats`).then((r) => r.json());

// Get events by type
const loanCreated = await fetch(`${API_BASE}/events/LoanCreated`).then((r) =>
  r.json()
);

// Get user's events
const userEvents = await fetch(`${API_BASE}/events/user/0xYourAddress`).then(
  (r) => r.json()
);
```

## 🏠 **Hosting Options**

### **Option 1: Single VPS (Recommended for MVP)**

**Cost: $5-20/month**

**Providers:**

- DigitalOcean Droplet ($5/month)
- AWS EC2 t3.micro ($8/month)
- Hetzner Cloud ($4/month)
- Linode ($5/month)

**Setup:**

```bash
# 1. SSH into your server
ssh root@your-server-ip

# 2. Install Docker
curl -fsSL https://get.docker.com -o get-docker.sh
sh get-docker.sh

# 3. Clone your repo
git clone https://github.com/yourusername/neuro-graph
cd neuro-graph/deployment

# 4. Deploy
./deploy.sh
```

### **Option 2: Docker Containers (Same Server)**

Run both services using Docker Compose:

```bash
cd deployment
docker-compose up -d
```

Services will run on:

- **Indexer**: Background process
- **API**: http://localhost:3001
- **Nginx**: http://localhost:80 (reverse proxy)

### **Option 3: Separate Servers (Production)**

**Cost: $10-40/month**

- **Server 1**: Indexer + Database
- **Server 2**: API Server + Load Balancer

### **Option 4: Cloud Services**

- **Railway**: $5/month per service
- **Render**: $7/month per service
- **Fly.io**: $3-10/month
- **Heroku**: $7/month per dyno

## 🔧 **Quick Local Test**

Test your current setup:

```bash
# Test API endpoints
curl http://localhost:3001/health
curl http://localhost:3001/events
curl http://localhost:3001/loans
curl http://localhost:3001/stats
```

## 🌍 **Production Deployment Steps**

### **1. Choose a VPS Provider**

- **DigitalOcean** (easiest)
- **AWS EC2** (most features)
- **Hetzner** (cheapest)

### **2. Set Up Domain (Optional)**

- Buy domain from Namecheap/GoDaddy
- Point A record to your server IP
- Update `nginx.conf` with your domain

### **3. Deploy**

```bash
# On your server
git clone https://github.com/yourusername/neuro-graph
cd neuro-graph/deployment
./deploy.sh
```

### **4. Configure SSL (Production)**

```bash
# Install Certbot
sudo apt install certbot python3-certbot-nginx

# Get SSL certificate
sudo certbot --nginx -d your-domain.com
```

### **5. Set Up Monitoring**

```bash
# View logs
docker-compose logs -f

# Monitor resources
htop
df -h
```

## 📱 **Frontend Integration**

Create a simple React component:

```jsx
// components/NeuroLendStats.jsx
import { useState, useEffect } from "react";

function NeuroLendStats() {
  const [stats, setStats] = useState(null);
  const [events, setEvents] = useState([]);

  useEffect(() => {
    // Fetch stats
    fetch("http://your-domain.com/api/stats")
      .then((r) => r.json())
      .then(setStats);

    // Fetch recent events
    fetch("http://your-domain.com/api/events?limit=10")
      .then((r) => r.json())
      .then((data) => setEvents(data.events));
  }, []);

  if (!stats) return <div>Loading...</div>;

  return (
    <div className="neurolend-stats">
      <h2>NeuroLend Stats</h2>
      <div className="stats-grid">
        <div>Total Events: {stats.total_events}</div>
        <div>Total Loans: {stats.total_loans}</div>
        <div>Active Loans: {stats.active_loans}</div>
      </div>

      <h3>Recent Events</h3>
      <ul>
        {events.map((event) => (
          <li key={`${event.transaction_hash}-${event.log_index}`}>
            {event.event_name} - Block {event.block_number}
          </li>
        ))}
      </ul>
    </div>
  );
}
```

## 🎯 **Your API Endpoints**

| Endpoint                  | Description          | Example Response                |
| ------------------------- | -------------------- | ------------------------------- |
| `GET /health`             | Health check         | `{"status": "healthy"}`         |
| `GET /events`             | All events           | `{"events": [...], "total": 2}` |
| `GET /events/LoanCreated` | Loan creation events | `{"events": [...]}`             |
| `GET /loans`              | All loans            | `[{"loan_id": "1", ...}]`       |
| `GET /stats`              | Statistics           | `{"total_events": 2, ...}`      |

## 🚀 **Next Steps**

1. **Deploy to a VPS** ($5/month)
2. **Connect your frontend** to the API
3. **Add real-time updates** (WebSocket)
4. **Scale as needed**

## 💡 **Pro Tips**

- Start with a single $5 VPS
- Use Docker for easy deployment
- Monitor logs with `docker-compose logs -f`
- Set up automatic backups
- Use environment variables for configuration

## 🆘 **Need Help?**

If you need assistance with deployment:

1. Choose your hosting provider
2. Set up the VPS
3. Run the deployment script
4. Test the endpoints
5. Connect your frontend

Your NeuroLend indexer is already working and finding events! 🎉
