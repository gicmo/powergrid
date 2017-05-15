<template>
  <div>
    <div class="row">
      <div class="col">
        <div id="graph-current" class="graph"></div>
      </div>
    </div>
  </div>
</template>

<script>
  import * as d3 from 'd3'

  export default {
    name: 'PowerGraph',
    props: ['log'],
    mounted () {
      const margin = {top: 20, right: 30, bottom: 30, left: 50}
      const width = 640
      const height = 150

      const log = this.log

      const energyFull = log[0]['energy-full'] / (1000 * 1000) // We want Watt

      const data = log.map(l => {
        return {
          timestamp: new Date(1988, 3, 15, 0, 0, 0, l['time-ms']),
          energy: l.energy / (1000 * 1000)
        }
      })

      const x = d3.time.scale()
                  .range([0, width])

      const y = d3.scale.linear()
                  .range([height, 0])
                  .domain([0, energyFull])

      const y1 = d3.scale.linear()
                   .range([height, 0])
                   .domain([0, 20])

      const ax = d3.svg.axis()
                   .scale(x)
                   .orient('bottom')
                   .tickFormat(d3.time.format('%H:%M'))

      const ay = d3.svg.axis()
                   .scale(y)
                   .orient('left').ticks(4)

      const ay1 = d3.svg.axis()
                    .scale(y1)
                    .orient('right').ticks(5)

      const svg = d3.select('#graph-current').append('svg')
                    .attr('width', width + margin.left + margin.right)
                    .attr('height', height + margin.top + margin.bottom)
                    .append('g')
                    .attr('transform', 'translate(' + margin.left + ',' + margin.top + ')')

      svg.append('g')
                    .attr('class', 'x axis')
                    .attr('id', 'x')
                    .attr('transform', 'translate(0,' + height + ')')
                    .call(ax)

      svg.append('g')
                    .attr('class', 'y axis')
                    .attr('id', 'y0')
                    .call(ay)
                    .append('text')
                    .attr('transform', 'rotate(-90)')
                    .attr('y', 6)
                    .attr('dy', '.71em')
                    .style('text-anchor', 'end')
                    .text('Energy')

      svg.append('g')
                    .attr('class', 'y axis')
                    .attr('id', 'y1')
                    .attr('transform', 'translate(' + (width - margin.right / 2.0) + ' ,0)')
                    .call(ay1)
                    .append('text')
                    .attr('transform', 'rotate(-90)')
                    .attr('dy', '-.71em')
                    .style('text-anchor', 'end')
                    .text('Power')

      const lineEnergy = d3.svg.line()
                            .x(d => { return x(d.timestamp) })
                            .y(d => { return y(d.energy) })

      x.domain(d3.extent(data, d => { return d.timestamp }))
      y.domain(d3.extent(data, d => { return d.energy }))

      svg.append('clipPath')
                            .attr('id', 'clip')
                            .append('rect')
                            .attr('x', '0')
                            .attr('y', '0')
                            .attr('width', width - margin.right / 2.0)
                            .attr('height', height)

      svg.append('path')
                            .datum(data)
                            .attr('id', 'line-energy')
                            .attr('class', 'line data-energy')
                            .attr('d', lineEnergy)
                            .attr('clip-path', 'url(#clip)')

      // svg.append('path')
      //   .datum(data)
      //   .attr('id', 'line-power')
      //   .attr('class', 'line data-power')
      //   .attr('d', line_power)
      //   .attr('clip-path', 'url(#clip)');

      svg.select('#x').call(ax)
      svg.select('#y0').call(ay)
    }
  }

</script>
