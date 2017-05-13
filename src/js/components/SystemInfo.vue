<template>
  <div>

    <div class="row">
      <div class="col offset-md-1 col-md-1" style="text-align: right">
        <h3>
          <i class="fa fa-laptop fa-3" aria-hidden="true"></i>
        </h3>
      </div>
      <div class="col col-md-8">
        <span class="sysinfo">
          {{ info.hardware.vendor }}
          {{ info.hardware.name }}
        </span>
      </div>
    </div>

    <div class="row">
      <div class="col offset-md-1 col-md-1" style="text-align: right">
        <i class="fa fa-battery-three-quarters fa-2" aria-hidden="true"></i>
      </div>
      <div class="col col-md-8">
        <span class="sysinfo">
          <div v-if="info.hardware.batteries.length > 0">
               <div v-for="bat in info.hardware.batteries">
                 <small>{{ bat.name }}</small> {{ bat.vendor }}: {{bat['energy-full']}} Wh
                 <small> (design: {{bat['energy-full-design']}} Wh)</small> <br>
               </div>
          </div>
          <div v-else >
          N/A
          </div>
        </span>
      </div>
    </div>

    <div class="row">
      <div class="col offset-md-1 col-md-1" style="text-align: right">
        <i class="fa fa-cogs fa-2" aria-hidden="true"></i>
      </div>
      <div class="col col-md-8">
        <span class="sysinfo">
          Bios Version: {{ info.hardware.bios.version }} <br>
          Bios Date: {{ info.hardware.bios.date }}
        </span>
      </div>
    </div>

    <div class="row">
      <div class="col offset-md-1 col-md-1" style="text-align: right">
        <i class="fa fa-microchip fa-2" aria-hidden="true"></i>
      </div>
      <div class="col col-md-8">
        <span class="sysinfo">
          CPU: {{ info.hardware.cpu.info[0] }} <br>
          RAM: {{ info.hardware.memory.total }}
        </span>
      </div>
    </div>

    <div class="row">
      <div class="col offset-md-1 col-md-1" style="text-align: right">
        <i class="fa fa-linux fa-2" aria-hidden="true"></i>
      </div>
      <div class="col col-md-8">
        <span class="sysinfo">
          OS: {{ info.software.os.type }} <br>
          Kernel: {{  info.software.os.kernel }} <br>
          GNOME:  {{  info.software.gnome.version }}
          ({{  info.software.gnome.date }})
          from  {{  info.software.gnome.distributor }}
        </span>
      </div>
    </div>

  </div>
</template>

<script>
  export default {
    name: 'SystemInfo',
    props: ['info']
  }
</script>

<style scoped>
  h1, h2 {
    font-weight: normal;
  }

  ul {
    list-style-type: none;
    padding: 0;
  }

  li {
    display: inline-block;
    margin: 0 10px;
  }

  a {
    color: #42b983;
  }
</style>
