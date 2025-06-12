// filepath: /netpulse/netpulse/static/app.js
document.addEventListener('DOMContentLoaded', function() {
    const chartContainer = document.getElementById('measurementChart');
    const dateLabelDiv = document.getElementById('dateLabel');
    const rangeButtons = document.querySelectorAll('.range-selector button');
    let currentRange = 'day';

    function fetchAndRender(range = 'day') {
        fetch(`/measurements?range=${range}`)
            .then(response => response.json())
            .then(data => {
                renderChart(data, range);
            })
            .catch(error => console.error('Error fetching data:', error));
    }

    rangeButtons.forEach(btn => {
        btn.addEventListener('click', () => {
            rangeButtons.forEach(b => b.classList.remove('active'));
            btn.classList.add('active');
            currentRange = btn.getAttribute('data-range');
            fetchAndRender(currentRange);
        });
    });

    function renderChart(data, range) {
        if (!chartContainer) {
            console.error("No chart container found!");
            return;
        }
        const ctx = document.createElement('canvas');
        chartContainer.innerHTML = ''; // Clear previous chart
        chartContainer.appendChild(ctx);

        // Determine the date or date range for the x-axis title
        let xAxisTitle = '';
        if (data.length > 0) {
            const first = data[0].timestamp.split('T')[0];
            const last = data[data.length - 1].timestamp.split('T')[0];
            xAxisTitle = (first === last) ? first : `${first} – ${last}`;
        }

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
                            text: xAxisTitle, // plain text only!
                            font: {
                                weight: 'bold', // makes it bold
                                size: 16
                            },
                            padding: { top: 18 } // more space above
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

        if (dateLabelDiv) {
            dateLabelDiv.textContent = '';
        }
    }

    // Add this for the Export Data button
    const exportBtn = document.getElementById('exportPdf');
    if (exportBtn) {
        exportBtn.addEventListener('click', function() {
            window.open('/export/pdf', '_blank');
        });
    }

    // Initial load
    fetchAndRender(currentRange);
});