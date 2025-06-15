// filepath: /netpulse/netpulse/static/app.js
document.addEventListener('DOMContentLoaded', function() {
    const chartContainer = document.getElementById('measurementChart');
    const dateLabelDiv = document.getElementById('dateLabel');
    const rangeButtons = document.querySelectorAll('.range-selector button');
    let currentRange = 'day';
    window.chartInstance = null; // Make chartInstance global

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
        console.log("renderChart called with range:", range);
        console.log("Data received:", data);
        
        // Clear and create new canvas
        const chartContainer = document.getElementById('chartContainer');
        chartContainer.innerHTML = '';
        const canvas = document.createElement('canvas');
        canvas.id = 'myChart';
        canvas.width = 1000;
        canvas.height = 500;
        chartContainer.appendChild(canvas);
        
        // Check if data exists and has proper format
        if (!data || !Array.isArray(data) || data.length === 0) {
            console.error("Invalid or empty data received");
            chartContainer.innerHTML = '<div style="text-align: center; padding: 20px; color: red;">No data available for this range</div>';
            return;
        }
        
        // Log the first data point to check format
        console.log("First data point:", data[0]);
        
        // Clean up old chart instance if it exists
        if (window.chartInstance) {
            window.chartInstance.destroy();
        }
        
        // Choose appropriate time unit
        let timeUnit = 'hour';
        if (range === 'week' || range === 'month') timeUnit = 'day';
        if (range === 'year') timeUnit = 'month';
        
        console.log("Using time unit:", timeUnit);
        
        try {
            const ctx = canvas.getContext('2d');
            window.chartInstance = new Chart(ctx, {
                type: 'line',
                data: {
                    datasets: [{
                        label: 'Bandwidth',
                        data: data.map(d => {
                            // Support both possible data formats
                            return {
                                x: d.timestamp || d.x,
                                y: d.value || d.y
                            };
                        }),
                        borderColor: '#88c0d0',
                        backgroundColor: 'rgba(136,192,208,0.2)',
                        tension: 0.2,
                        pointRadius: 3
                    }]
                },
                options: {
                    responsive: true,
                    maintainAspectRatio: false,
                    scales: {
                        x: {
                            type: 'time',
                            time: {
                                unit: timeUnit
                            },
                            title: {
                                display: true,
                                text: 'Time'
                            }
                        },
                        y: {
                            title: {
                                display: true,
                                text: 'Mbit/s'
                            },
                            beginAtZero: true
                        }
                    }
                }
            });
            console.log("Chart created successfully");
        } catch (error) {
            console.error("Error creating chart:", error);
        }
    }

    if (dateLabelDiv) {
        dateLabelDiv.textContent = '';
    }
});
// Event listeners for download and export buttons
document.getElementById('downloadChart').addEventListener('click', function(e) {
    if (window.chartInstance) {
        try {
            // Direct user interaction with explicit canvas reference
            const dataUrl = window.chartInstance.canvas.toDataURL('image/jpeg', 1.0);
            const link = document.createElement('a');
            link.href = dataUrl;
            link.download = 'chart.jpg';
            document.body.appendChild(link); // Important for some browsers
            link.click();
            document.body.removeChild(link);
        } catch (err) {
            console.error("Export failed:", err);
            alert("Export failed: " + err.message);
        }
    } else {
        alert('Please generate the chart first.');
    }
});

document.getElementById('exportChartPdf').addEventListener('click', function(e) {
    if (window.chartInstance) {
        try {
            // Direct user interaction with explicit canvas reference
            const dataUrl = window.chartInstance.canvas.toDataURL('image/jpeg', 1.0);
            const pdf = new window.jspdf.jsPDF();
            pdf.addImage(dataUrl, 'JPEG', 10, 10, 180, 100);
            pdf.save('chart.pdf');
        } catch (err) {
            console.error("PDF Export failed:", err);
            alert("PDF Export failed: " + err.message);
        }
    } else {
        alert('Please generate the chart first.');
    }
});

// Initial load
fetchAndRender(currentRange);