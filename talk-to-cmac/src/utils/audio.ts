/**
 * Audio Recording and Playback Utilities
 *
 * Handles browser-based audio recording and playback
 */

export interface RecordingOptions {
  sampleRate?: number;
  channels?: number;
  bitsPerSample?: number;
  maxDuration?: number;
}

export interface AudioRecorder {
  start: () => Promise<void>;
  stop: () => Promise<Uint8Array>;
  pause: () => void;
  resume: () => void;
  isRecording: boolean;
  isPaused: boolean;
  duration: number;
}

/**
 * Create an audio recorder instance
 */
export async function createAudioRecorder(
  options: RecordingOptions = {}
): Promise<AudioRecorder> {
  const {
    sampleRate = 16000,
    channels = 1,
    bitsPerSample = 16,
    maxDuration = 60000, // 60 seconds
  } = options;

  let mediaRecorder: MediaRecorder | null = null;
  let audioChunks: Blob[] = [];
  let stream: MediaStream | null = null;
  let startTime: number = 0;
  let pausedTime: number = 0;
  let isPaused = false;
  let maxDurationTimer: number | null = null;

  const recorder: AudioRecorder = {
    start: async () => {
      try {
        // Request microphone access
        stream = await navigator.mediaDevices.getUserMedia({
          audio: {
            channelCount: channels,
            sampleRate: sampleRate,
            echoCancellation: true,
            noiseSuppression: true,
            autoGainControl: true,
          },
        });

        // Create MediaRecorder
        const mimeType = getSupportedMimeType();
        mediaRecorder = new MediaRecorder(stream, { mimeType });

        audioChunks = [];
        startTime = Date.now();
        pausedTime = 0;
        isPaused = false;

        // Collect audio data
        mediaRecorder.ondataavailable = (event) => {
          if (event.data.size > 0) {
            audioChunks.push(event.data);
          }
        };

        // Start recording
        mediaRecorder.start(100); // Collect data every 100ms

        // Set max duration timer
        if (maxDuration > 0) {
          maxDurationTimer = setTimeout(() => {
            recorder.stop();
          }, maxDuration);
        }
      } catch (error) {
        throw new Error(`Failed to start recording: ${error}`);
      }
    },

    stop: async () => {
      return new Promise<Uint8Array>((resolve, reject) => {
        if (!mediaRecorder) {
          reject(new Error('No active recording'));
          return;
        }

        mediaRecorder.onstop = async () => {
          try {
            // Clear max duration timer
            if (maxDurationTimer) {
              clearTimeout(maxDurationTimer);
              maxDurationTimer = null;
            }

            // Stop all tracks
            if (stream) {
              stream.getTracks().forEach((track) => track.stop());
              stream = null;
            }

            // Create blob from chunks
            const audioBlob = new Blob(audioChunks, {
              type: getSupportedMimeType(),
            });

            // Convert to WAV format
            const wavData = await convertToWav(
              audioBlob,
              sampleRate,
              channels,
              bitsPerSample
            );

            mediaRecorder = null;
            audioChunks = [];

            resolve(wavData);
          } catch (error) {
            reject(new Error(`Failed to process recording: ${error}`));
          }
        };

        mediaRecorder.stop();
      });
    },

    pause: () => {
      if (mediaRecorder && mediaRecorder.state === 'recording') {
        mediaRecorder.pause();
        pausedTime = Date.now();
        isPaused = true;
      }
    },

    resume: () => {
      if (mediaRecorder && mediaRecorder.state === 'paused') {
        mediaRecorder.resume();
        startTime += Date.now() - pausedTime;
        isPaused = false;
      }
    },

    get isRecording() {
      return mediaRecorder !== null && mediaRecorder.state === 'recording';
    },

    get isPaused() {
      return isPaused;
    },

    get duration() {
      if (!startTime) return 0;
      if (isPaused) return pausedTime - startTime;
      return Date.now() - startTime;
    },
  };

  return recorder;
}

/**
 * Get supported MIME type for MediaRecorder
 */
function getSupportedMimeType(): string {
  const types = [
    'audio/webm;codecs=opus',
    'audio/webm',
    'audio/ogg;codecs=opus',
    'audio/mp4',
  ];

  for (const type of types) {
    if (MediaRecorder.isTypeSupported(type)) {
      return type;
    }
  }

  return '';
}

/**
 * Convert audio blob to WAV format
 */
async function convertToWav(
  blob: Blob,
  sampleRate: number,
  _channels: number,
  bitsPerSample: number
): Promise<Uint8Array> {
  // Create audio context
  const audioContext = new (window.AudioContext ||
    (window as any).webkitAudioContext)({
    sampleRate,
  });

  try {
    // Decode audio data
    const arrayBuffer = await blob.arrayBuffer();
    const audioBuffer = await audioContext.decodeAudioData(arrayBuffer);

    // Resample if needed
    const resampledBuffer =
      audioBuffer.sampleRate !== sampleRate
        ? await resampleAudioBuffer(audioBuffer, sampleRate)
        : audioBuffer;

    // Convert to WAV
    return audioBufferToWav(resampledBuffer, bitsPerSample);
  } finally {
    await audioContext.close();
  }
}

/**
 * Resample audio buffer to target sample rate
 */
async function resampleAudioBuffer(
  audioBuffer: AudioBuffer,
  targetSampleRate: number
): Promise<AudioBuffer> {
  const _channels = audioBuffer.numberOfChannels; // Store for clarity
  const offlineContext = new OfflineAudioContext(
    _channels,
    (audioBuffer.duration * targetSampleRate),
    targetSampleRate
  );

  const source = offlineContext.createBufferSource();
  source.buffer = audioBuffer;
  source.connect(offlineContext.destination);
  source.start(0);

  return await offlineContext.startRendering();
}

/**
 * Convert AudioBuffer to WAV format
 */
function audioBufferToWav(
  audioBuffer: AudioBuffer,
  bitsPerSample: number = 16
): Uint8Array {
  const numberOfChannels = audioBuffer.numberOfChannels;
  const sampleRate = audioBuffer.sampleRate;
  const length = audioBuffer.length * numberOfChannels * (bitsPerSample / 8);

  const buffer = new ArrayBuffer(44 + length);
  const view = new DataView(buffer);

  // WAV header
  writeString(view, 0, 'RIFF');
  view.setUint32(4, 36 + length, true);
  writeString(view, 8, 'WAVE');
  writeString(view, 12, 'fmt ');
  view.setUint32(16, 16, true); // Subchunk1Size
  view.setUint16(20, 1, true); // AudioFormat (PCM)
  view.setUint16(22, numberOfChannels, true);
  view.setUint32(24, sampleRate, true);
  view.setUint32(28, sampleRate * numberOfChannels * (bitsPerSample / 8), true);
  view.setUint16(32, numberOfChannels * (bitsPerSample / 8), true);
  view.setUint16(34, bitsPerSample, true);
  writeString(view, 36, 'data');
  view.setUint32(40, length, true);

  // Write audio data
  const channels: Float32Array[] = [];
  for (let i = 0; i < numberOfChannels; i++) {
    channels.push(audioBuffer.getChannelData(i));
  }

  let offset = 44;
  for (let i = 0; i < audioBuffer.length; i++) {
    for (let channelIndex = 0; channelIndex < numberOfChannels; channelIndex++) {
      const sample = Math.max(-1, Math.min(1, channels[channelIndex][i]));
      if (bitsPerSample === 16) {
        view.setInt16(offset, sample < 0 ? sample * 0x8000 : sample * 0x7fff, true);
        offset += 2;
      } else {
        view.setUint8(offset, (sample + 1) * 0x7f);
        offset += 1;
      }
    }
  }

  return new Uint8Array(buffer);
}

/**
 * Write string to DataView
 */
function writeString(view: DataView, offset: number, string: string): void {
  for (let i = 0; i < string.length; i++) {
    view.setUint8(offset + i, string.charCodeAt(i));
  }
}

/**
 * Create an audio player for playing audio responses
 */
export class AudioPlayer {
  private audio: HTMLAudioElement | null = null;
  private currentUrl: string | null = null;

  /**
   * Play audio from Uint8Array (MP3 format)
   */
  async play(audioData: Uint8Array): Promise<void> {
    return new Promise((resolve, reject) => {
      try {
        // Stop current playback
        this.stop();

        // Create blob and URL
        const blob = new Blob([audioData], { type: 'audio/mpeg' });
        this.currentUrl = URL.createObjectURL(blob);

        // Create audio element
        this.audio = new Audio(this.currentUrl);

        // Event listeners
        this.audio.onended = () => {
          this.stop();
          resolve();
        };

        this.audio.onerror = (error) => {
          this.stop();
          reject(new Error(`Audio playback failed: ${error}`));
        };

        // Start playback
        this.audio.play();
      } catch (error) {
        this.stop();
        reject(new Error(`Failed to play audio: ${error}`));
      }
    });
  }

  /**
   * Stop current playback
   */
  stop(): void {
    if (this.audio) {
      this.audio.pause();
      this.audio.src = '';
      this.audio = null;
    }

    if (this.currentUrl) {
      URL.revokeObjectURL(this.currentUrl);
      this.currentUrl = null;
    }
  }

  /**
   * Pause current playback
   */
  pause(): void {
    if (this.audio) {
      this.audio.pause();
    }
  }

  /**
   * Resume paused playback
   */
  resume(): void {
    if (this.audio) {
      this.audio.play();
    }
  }

  /**
   * Get current playback time
   */
  get currentTime(): number {
    return this.audio?.currentTime ?? 0;
  }

  /**
   * Get total duration
   */
  get duration(): number {
    return this.audio?.duration ?? 0;
  }

  /**
   * Check if audio is currently playing
   */
  get isPlaying(): boolean {
    return this.audio !== null && !this.audio.paused;
  }
}

/**
 * Format duration in seconds to MM:SS
 */
export function formatDuration(seconds: number): string {
  const mins = Math.floor(seconds / 60);
  const secs = Math.floor(seconds % 60);
  return `${mins.toString().padStart(2, '0')}:${secs.toString().padStart(2, '0')}`;
}
