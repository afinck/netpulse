// filepath: /netpulse/netpulse/static/app.js
document.addEventListener('DOMContentLoaded', function() {
    const chartContainer = document.getElementById('measurementChart');
    const dateLabelDiv = document.getElementById('dateLabel');
    const rangeButtons = document.querySelectorAll('.range-selector button');
    let currentRange = 'day';
    let chartInstance = null; // Store the chart instance here

    function fetchAndRender(range = 'day') {
        fetch(`/measurements?range=${range}`)
            .then(response => response.json())
            .then(data => {
                renderChart(data, range);
            })
            .catch(error => console.error('Error fetching data:', error));
    }
    // Range button logic
    document.querySelectorAll('.range-btn').forEach(btn => {
        btn.addEventListener('click', function() {
            document.querySelectorAll('.range-btn').forEach(b => b.classList.remove('active'));
            btn.classList.add('active');
            fetchAndRender(btn.dataset.range);
        });
    });


    function renderChart(data, range) {
        const chartContainer = document.getElementById('chartContainer');
        chartContainer.innerHTML = '';
        const canvas = document.createElement('canvas');
        canvas.id = 'myChart';
        canvas.width = 800;
        canvas.height = 400;
        chartContainer.appendChild(canvas);

        if (window.chartInstance) {
            window.chartInstance.destroy();
        }

        let timeUnit = 'hour';
        if (range === 'week' || range === 'month') {
            timeUnit = 'day';
        } else if (range === 'year') {
            timeUnit = 'month';
        }

        const ctx = canvas.getContext('2d');
        chartInstance = new Chart(ctx, {
            type: 'line',
            data: {
                datasets: [{
                    label: 'Bandwidth',
                    data: data.map(d => ({ x: d.timestamp, y: d.value })),
                    borderColor: '#88c0d0',
                    backgroundColor: 'rgba(136,192,208,0.2)',
                    tension: 0.2,
                    pointRadius: 2
                }]
            },
            options: {
                responsive: true,
                scales: {
                    x: {
                        type: 'time',
                        time: {
                            unit: timeUnit,
                            displayFormats: {
                                hour: 'HH:mm',
                                day: 'MMM d',
                                month: 'MMM yyyy'
                            }
                        },
                        title: {
                            display: true,
                            text: 'Date'
                        }
                    },
                    y: {
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

    // Add this for the Download Chart button
    const downloadBtn = document.getElementById('downloadChart');
    if (downloadBtn) {
        downloadBtn.addEventListener('click', function() {
            if (chartInstance) {
                const link = document.createElement('a');
                link.href = chartInstance.toBase64Image({ type: 'image/jpeg', quality: 1.0, pixelRatio: 2 }); // pixelRatio for sharpness
                link.download = 'chart.jpg';
                link.click();
            } else {
                alert('Please generate the chart first.');
            }
        });
    }

    // Add this for the Export Chart as PDF button
    const exportChartBtn = document.getElementById('exportChartPdf');
    if (exportChartBtn) {
        exportChartBtn.addEventListener('click', function() {
            if (chartInstance) {
                const imgData = chartInstance.toBase64Image();
                const pdf = new window.jspdf.jsPDF();
                pdf.addImage(imgData, 'PNG', 10, 10, 180, 100); // adjust size/position as needed
                pdf.save('chart.pdf');
            } else {
                alert('Please generate the chart first.');
            }
        });
    }

    // Initial load
    fetchAndRender(currentRange);
});