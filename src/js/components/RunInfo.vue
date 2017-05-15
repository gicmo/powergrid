<template>
  <div>
    <div class="row">
      <div class="col offset-md-1 col-md-1" style="text-align: right">
        <h3>
          <i class="fa fa-book" aria-hidden="true"></i>
        </h3>
      </div>
      <div class="col col-md-8">
        <span class="sysinfo">
          {{ info.name }} <br>
          {{ info.conditoon }}
        </span>
      </div>
    </div>

    <div class="row">
      <div class="col offset-md-1 col-md-1" style="text-align: right">
        <h3>
          <i class="fa fa-battery-empty fa-3" aria-hidden="true"></i>
        </h3>
      </div>
      <div class="col col-md-8">
        <span class="sysinfo">
          Life (est): {{ info.life }} h
        </span>
      </div>
    </div>

  </div>
</template>

<script>
  export default {
    name: 'RunInfo',
    props: ['run'],
    computed: {
      info () {
        let condition

        if ('duration' in this.run) {
          condition = 'Duration: ' + this.run.duration + ' sec'
        } else if ('until-percent' in this.run) {
          condition = 'Until: ' + this.run['until-percent'] + ' %'
        } else {
          condition = 'Unknown test condition'
        }

        const estLife = this.run['estimated-life']
        const estHour = Math.round(estLife / 3600)
        const estMins = Math.round((estLife % 3600) / 60)

        return {
          name: this.run['test-name'],
          condition: condition,
          life: `${estHour}:${estMins}`
        }
      }
    }
  }
</script>

<style scoped>

</style>
