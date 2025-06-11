// filepath: /netpulse/netpulse/static/app.js
document.addEventListener('DOMContentLoaded', function() {
    const fetchDataButton = document.getElementById('fetch-data');
    const chartContainer = document.getElementById('chart-container');

    fetchDataButton.addEventListener('click', function() {
        fetch('/api/measurements')
            .then(response => response.json())
            .then(data => {
                renderChart(data);
            })
            .catch(error => console.error('Error fetching data:', error));
    });

    function renderChart(data) {
        const ctx = document.createElement('canvas');
        chartContainer.innerHTML = ''; // Clear previous chart
        chartContainer.appendChild(ctx);

        const chart = new Chart(ctx, {
            type: 'line',
            data: {
                labels: data.map(entry => entry.timestamp),
                datasets: [{
                    label: 'Measurements',
                    data: data.map(entry => entry.value),
                    borderColor: 'rgba(75, 192, 192, 1)',
                    borderWidth: 1,
                    fill: false
                }]
            },
            options: {
                responsive: true,
                scales: {
                    x: {
                        type: 'time',
                        time: {
                            unit: 'minute'
                        }
                    },
                    y: {
                        beginAtZero: true
                    }
                }
            }
        });
    }
});