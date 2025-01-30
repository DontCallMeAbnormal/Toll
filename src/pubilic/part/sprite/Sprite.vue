<template>
  <div>
    <div>
      <h3>音乐节奏</h3>
      <div id="music-rhythm">
        <div 
          class="progress-bar" 
          @wheel="handleWheel"
          :style="{ width: progressBarWidth + 'px', height: progressBarHeight + 'px' }"
        >
          <div 
            v-for="(note, index) in musicRhythm" 
            :key="index" 
            class="note" 
            :style="{ width: noteWidth(note), left: notePosition(note), top: noteTop(note) }"
            @mouseenter="hoverNote(note.key, $event)"
            @mouseleave="hoverNote(null)"
          ></div>
        </div>
        <div v-if="hoveredKey !== null" class="hovered-key" :style="{ left: mouseX + 'px', top: mouseY + 'px' }">
          Key: {{ hoveredKey }}
        </div>
      </div>
    </div>
    <div>
      <h3>动画预览</h3>
      <div id="animation-preview">
      </div>
    </div>
    <div>
      <div class="image-grid">
        <div v-for="(image, index) in images" :key="index" class="image-upload">
          <div class="button-container">
            <el-upload
              class="upload-demo"
              action="#"
              :show-file-list="false"
              :auto-upload="false"
              :on-change="(file) => handleImageUpload(file, index)"
              accept="image/*"
            >
              <template v-slot:trigger>
                <el-button type="primary">选择图片</el-button>
              </template>
            </el-upload>
            <span class="delete-icon" @click="removeImage(index)">×</span>
          </div>
          <el-select v-model="image.condition" placeholder="出现条件" class="condition-select">
            <el-option label="大于" value="greater_than" />
            <el-option label="等于" value="equal_to" />
            <el-option label="小于" value="less_than" />
            <el-option label="范围" value="range" />
            <el-option label="任何值" value="any_value" />
          </el-select>

          <el-input
            v-if="image.condition !== 'any_value'"
            v-model="image.value"
            :placeholder="getImageValuePlaceholder()"
            class="image-value-input"
            :disabled="image.condition === 'any_value'"
            :rules="getImageValueRules()"
            @input="validateInput(index)"
          />

          <div v-if="image.url" class="thumbnail">
            <img :src="image.url" alt="Thumbnail" />
          </div>
          <div v-if="validationErrors[index]" class="error-message">
            {{ validationErrors[index] }}
          </div>
        </div>
      </div>
      <div>
        <el-button type="primary" @click="addImageUpload">添加图片上传框</el-button>
      </div>
    </div>
    <div class="bottom-right-buttons">
      <el-upload
        class="upload-demo"
        action="#"
        :show-file-list="false"
        :auto-upload="false"
        :on-change="handleMidiUpload"
        accept=".mid,.midi"
      >
        <template v-slot:trigger>
          <el-button type="primary">选择MIDI文件</el-button>
        </template>
      </el-upload>
      <el-button type="success" @click="generateAnimation">生成动画</el-button>
    </div>
  </div>
</template>

<script>
import { ElUpload, ElButton, ElInput } from 'element-plus';
import { invoke } from '@tauri-apps/api/tauri';

export default {
  components: {
    ElUpload,
    ElButton,
    ElInput,
  },
  data() {
    return {
      midiFile: null,
      images: [],
      musicRhythm: [],
      totalTime: 0,
      progressBarWidth: 1000, // 初始宽度
      progressBarHeight: 100, // 初始高度
      validationErrors: {}, // 新增：用于存储每个输入框的校验状态
      hoveredKey: null, // 新增：用于存储当前悬停的 note 的 key 值
      mouseX: 0,
      mouseY: 0,
    };
  },
  methods: {
    // 处理 MIDI 文件上传
    async handleMidiUpload(file) {
      this.midiFile = file.raw;
      const arrayBuffer = await this.midiFile.arrayBuffer();
      const uint8Array = new Uint8Array(arrayBuffer);
      const midiData = Array.from(uint8Array);

      const rhythm = await invoke('parse_midi', { file: midiData });
      this.musicRhythm = JSON.parse(rhythm);
      this.calculateTotalTime();
    },
    // 计算总时间
    calculateTotalTime() {
      if (this.musicRhythm.length > 0) {
        const lastNote = this.musicRhythm[this.musicRhythm.length - 1];
        this.totalTime = lastNote.start_time + lastNote.duration;
        this.progressBarWidth = this.musicRhythm.length * 50
      }
    },
    // 处理图片上传
    handleImageUpload(file, index) {
      if (file) {
        this.images[index].file = file.raw;
        this.images[index].url = URL.createObjectURL(file.raw);
      }
    },
    // 添加图片上传框
    addImageUpload() {
      this.images.push({ id: '', file: null, url: '' });
    },
    // 生成动画
    generateAnimation() {
      let isValid = true;
      this.images.forEach((image, index) => {
        if (image.condition !== 'any_value') {
          const rules = this.getImageValueRules(image.condition);
          const value = image.value;
          let error = false;
          rules.forEach(rule => {
            if (rule.validator) {
              rule.validator(rule, value, (err) => {
                if (err) {
                  error = true;
                  this.validationErrors[index] = err.message; // 替换为直接赋值
                } else {
                  delete this.validationErrors[index]; // 替换为直接删除属性
                }
              });
            }
          });
          if (error) {
            isValid = false;
          }
        }
      });
      if (isValid) {
        // 根据上传的 MIDI 文件和图片生成动画
        invoke('generate_animation', { musicRhythm: this.musicRhythm, images: this.images })
          .then(response => {
            console.log('Animation generated:', response);
          })
          .catch(error => {
            console.error('Error generating animation:', error);
          });
      }
    },
    // 删除图片
    removeImage(index) {
      this.images.splice(index, 1);
    },
    // 计算音符宽度
    noteWidth(note) {
      if (note.duration <= 0) {
        return '0.01%';
      }
      return `${Math.min((note.duration / this.totalTime) * 100, 100)}%`; // 确保宽度不超过100%
    },
    // 计算音符位置
    notePosition(note) {
      return `${(note.start_time / this.totalTime) * 100}%`;
    },
    // 计算音符纵向位置
    noteTop(note) {
      // MIDI 键值范围通常是 0 到 127
      const keyHeight = this.progressBarHeight / 128;
      return `${(127 - note.key) * keyHeight}px`;
    },
    // 处理鼠标滚轮事件
    handleWheel(event) {
      const delta = Math.sign(event.deltaY);
      if (event.ctrlKey) {
        this.progressBarWidth = Math.max(100, this.progressBarWidth - delta * 50); // 最小宽度为100px
      } else if (event.altKey) {
        this.progressBarHeight = Math.max(50, this.progressBarHeight - delta * 50); // 最小高度为50px
      }
    },
    hoverNote(key, event) {
      if (key !== null) {
        this.hoveredKey = key;
        this.mouseX = event.clientX;
        this.mouseY = event.clientY;
      } else {
        this.hoveredKey = null;
      }
    },
    getImageValuePlaceholder() {
      const condition = this.images.find(image => image.condition !== 'any_value')?.condition;
      switch (condition) {
        case 'greater_than':
          return '请输入一个大于条件的数字';
        case 'equal_to':
          return '请输入一个等于条件的数字';
        case 'less_than':
          return '请输入一个小于条件的数字';
        case 'range':
          return '请输入一个长度为2的数字数组，例如 [1, 5]';
        default:
          return '';
      }
    },
    getImageValueRules(condition) {
      let rules = [];
      switch (condition) {
        case 'greater_than':
        case 'equal_to':
        case 'less_than':
          rules.push({
            required: true,
            message: '请输入一个有效的数字',
            trigger: 'blur',
            validator: (rule, value, callback) => {
              if (!value || isNaN(value)) {
                callback(new Error('请输入一个有效的数字'));
              } else {
                callback();
              }
            }
          });
          break;
        case 'range':
          rules.push({
            required: true,
            message: '请输入一个长度为2的数字数组',
            trigger: 'blur',
            validator: (rule, value, callback) => {
              try{
                value = JSON.parse(value);
              } catch (e) {
                callback(new Error('请输入一个长度为2的数字数组'));
              }
              if (!Array.isArray(value) || value.length !== 2 || value.some(isNaN)) {
                callback(new Error('请输入一个长度为2的数字数组'));
              } else {
                callback();
              }
            }
          });
          break;
        default:
          break;
      }
      return rules;
    },
    validateInput(index) {
      const image = this.images[index];
      if (image.condition !== 'any_value') {
        const rules = this.getImageValueRules(image.condition);
        const value = image.value;
        let error = false;
        rules.forEach(rule => {
          if (rule.validator) {
            rule.validator(rule, value, (err) => {
              if (err) {
                error = true;
                this.validationErrors[index] = err.message;
              } else {
                delete this.validationErrors[index];
              }
            });
          }
        });
        if (!error) {
          delete this.validationErrors[index];
        }
      }
    },
  },
};
</script>

<style>
.image-upload {
  display: flex;
  flex-direction: column; /* 修改为列方向 */
  align-items: flex-start; /* 左对齐 */
  margin-bottom: 10px;
  padding:0.5em;
  border: 1px solid #ccc;
}

.image-upload > div {
  margin-bottom: 0.5em;
}

.button-container {
  display: flex;
  flex-direction: row; /* 按钮在同一行 */
  width: 100%;
  justify-content: space-between;
  align-items: flex-start;
}

.image-id-input {
  margin-top: 1em; /* 图片ID输入框移到新的一行 */
}

.image-grid {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 10px;
}

.thumbnail {
  width: 100px;
  height: 100px;
  overflow: hidden;
  margin-top: 10px;
  background: #ccc;
}

.thumbnail img {
  width: 100%;
  height: 100%;
  object-fit: cover;
}

.bottom-right-buttons {
  position: fixed;
  bottom: 20px;
  right: 20px;
  display: flex;
  flex-direction: row; /* 确保按钮在同一行 */
  gap: 10px;
}

.progress-bar {
  position: relative;
  background-color: #ddd;
  margin-top: 20px;
}

.note {
  position: absolute;
  height: 1%; /* 修改为固定高度 */
  background-color: green;
}

.delete-icon {
  cursor: pointer;
  font-size: 1.2em;
  padding: 0 1em;
  color: red;
}
.delete-icon:hover {
  background-color: #fff;
}

.image-value-input.error {
  border-color: red;
}

.error-message {
  color: red;
  font-size: 0.9em;
  margin-top: 5px;
}

.hovered-key {
  position: absolute;
  background-color: rgba(255, 255, 255, 0.8);
  border: 1px solid #000;
  padding: 5px;
  color: #000;
  border-radius: 5px;
  z-index: 1000;
}
</style>