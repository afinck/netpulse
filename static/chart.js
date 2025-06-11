// filepath: /netpulse/netpulse/static/chart.js
document.addEventListener('DOMContentLoaded', function() {
    const ctx = document.getElementById('measurementsChart');
    if (!ctx) {
        console.error("Canvas element with id 'measurementsChart' not found.");
        return;
    }
    const myChart = new Chart(ctx, {
        type: 'line',
        data: {
            labels: [],
            datasets: [{
                label: 'Measurement Data',
                data: [],
                borderColor: 'rgba(75, 192, 192, 1)',
                backgroundColor: 'rgba(75, 192, 192, 0.2)',
                borderWidth: 1
            }]
        },
        options: {
            scales: {
                y: {
                    beginAtZero: true
                }
            }
        }
    });

    // Function to fetch data from the backend and update the chart
    async function fetchData() {
        const response = await fetch('/measurements');
        const data = await response.json();

        // If your backend returns an array of objects with timestamp and value:
        myChart.data.labels = data.map(entry => entry.timestamp);
        myChart.data.datasets[0].data = data.map(entry => entry.value);
        myChart.update();
    }

    fetchData();
});