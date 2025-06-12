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
                    label: 'Bandwidth (Mbit/s)',
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
                        bounds: 'ticks',
                        offset: false,
                        time: {
                            unit: 'minute',
                            displayFormats: {
                                millisecond: 'HH:mm',
                                second: 'HH:mm',
                                minute: 'HH:mm',
                                hour: 'HH:mm',
                                day: 'HH:mm',
                                week: 'HH:mm',
                                month: 'HH:mm',
                                quarter: 'HH:mm',
                                year: 'HH:mm'
                            },
                            tooltipFormat: 'yyyy-MM-dd HH:mm:ss'
                        },
                        title: {
                            display: true,
                            text: 'Timestamp (24h)'
                        },
                        ticks: {
                            callback: function(value) {
                                const date = new Date(value);
                                return date.toLocaleTimeString('en-GB', { hour: '2-digit', minute: '2-digit', hour12: false });
                            },
                            maxRotation: 0,
                            minRotation: 0,
                            autoSkip: true,
                            maxTicksLimit: 8,
                            major: {
                                enabled: false
                            }
                        }
                    },
                    y: {
                        beginAtZero: true,
                        title: {
                            display: true,
                            text: 'Mbit/s'
                        }
                    }
                }
            }
        });

        // Remove previous date labels if any
        const oldDateLabels = document.querySelectorAll('.chart-date-label');
        oldDateLabels.forEach(el => el.remove());

        // Find unique dates in the data, sorted
        const uniqueDates = [...new Set(data.map(entry => (entry.timestamp.split('T')[0] || entry.timestamp.split(' ')[0])))];

        // Create a single label with all dates, separated by " / " if more than one
        const dateLabel = document.createElement('div');
        dateLabel.className = 'chart-date-label';
        dateLabel.style.textAlign = 'center';
        dateLabel.style.marginTop = '16px';
        dateLabel.style.fontWeight = 'bold';
        dateLabel.style.fontSize = '1.1em';
        dateLabel.textContent = uniqueDates.join(' / ');

        // Insert the date label AFTER the chart canvas
        chartContainer.appendChild(dateLabel);
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