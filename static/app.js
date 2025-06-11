// filepath: /netpulse/netpulse/static/app.js
document.addEventListener('DOMContentLoaded', function() {
    const fetchDataButton = document.getElementById('exportPdf');
    const chartContainer = document.getElementById('measurementChart');

    function renderChart(data) {
        if (!chartContainer) {
            console.error("No chart container found!");
            return;
        }
        const ctx = document.createElement('canvas');
        chartContainer.innerHTML = ''; // Clear previous chart
        chartContainer.appendChild(ctx);

        new Chart(ctx, {
            type: 'line',
            data: {
                labels: data.map(entry => entry.timestamp),
                datasets: [{
                    label: 'Bandwidth (Mbit/s)', // <-- Make it clear!
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
                        beginAtZero: true,
                        title: {
                            display: true,
                            text: 'Mbit/s' // <-- Y-axis label
                        }
                    }
                }
            }
        });
    }

    function fetchAndRender() {
        fetch('/measurements')
            .then(response => response.json())
            .then(data => {
                renderChart(data);
            })
            .catch(error => console.error('Error fetching data:', error));
    }

    // Load chart on page load
    fetchAndRender();

    // Also allow manual refresh via button
    if (fetchDataButton) {
        fetchDataButton.addEventListener('click', fetchAndRender);
    }
});