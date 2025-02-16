const { execSync } = require('child_process');
const fs = require('fs');
const path = require('path');

class DeploymentErrorHandler {
  constructor() {
    this.logFile = path.join(__dirname, '../deployment-errors.log');
  }

  // Log deployment errors
  logError(error, context = {}) {
    const errorLog = {
      timestamp: new Date().toISOString(),
      error: error.message,
      stack: error.stack,
      ...context
    };

    fs.appendFileSync(
      this.logFile, 
      JSON.stringify(errorLog, null, 2) + '\n'
    );
  }

  // Rollback deployment if critical error occurs
  async rollback(deploymentType = 'kubernetes') {
    try {
      switch(deploymentType) {
        case 'kubernetes':
          execSync('kubectl rollout undo deployment/meme-coin-platform');
          break;
        case 'docker':
          execSync('docker-compose down && docker-compose up -d');
          break;
        default:
          throw new Error('Unsupported deployment type');
      }
      
      this.notifyAdmins('Automatic rollback initiated');
    } catch (rollbackError) {
      this.logError(rollbackError, { 
        action: 'Rollback', 
        deploymentType 
      });
    }
  }

  // Send notifications via multiple channels
  notifyAdmins(message, channels = ['slack', 'email']) {
    channels.forEach(channel => {
      switch(channel) {
        case 'slack':
          this.sendSlackNotification(message);
          break;
        case 'email':
          this.sendEmailNotification(message);
          break;
      }
    });
  }

  sendSlackNotification(message) {
    // Implement Slack webhook notification
    try {
      execSync(`curl -X POST -H 'Content-type: application/json' --data '{"text":"${message}"}' ${process.env.SLACK_WEBHOOK_URL}`);
    } catch (error) {
      this.logError(error, { action: 'Slack Notification' });
    }
  }

  sendEmailNotification(message) {
    // Implement email notification logic
    // Could use services like SendGrid or AWS SES
    console.log('Email Notification:', message);
  }

  // Comprehensive health check
  async performHealthCheck() {
    const checks = [
      { 
        name: 'Database Connection', 
        command: 'pg_isready -h $DB_HOST -p $DB_PORT -d $DB_NAME' 
      },
      { 
        name: 'Redis Connection', 
        command: 'redis-cli ping' 
      },
      { 
        name: 'Application Readiness', 
        command: 'curl -f http://localhost:3000/health' 
      }
    ];

    const results = [];

    for (const check of checks) {
      try {
        execSync(check.command);
        results.push({ 
          service: check.name, 
          status: 'HEALTHY' 
        });
      } catch (error) {
        results.push({ 
          service: check.name, 
          status: 'UNHEALTHY',
          error: error.message 
        });
        
        // Log and potentially trigger alerts for unhealthy services
        this.logError(error, { service: check.name });
      }
    }

    return results;
  }
}

module.exports = new DeploymentErrorHandler();
