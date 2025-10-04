# Engine Separation Refactoring Plan

## 목표 (Goals)

현재 Flappy Bird 게임에 강결합된 렌더링 엔진을 범용 게임 엔진으로 분리하여, 다양한 프로젝트(특히 2D Physics 구현)에서 재사용 가능한 구조로 리팩토링한다.

## 현재 문제점 (Current Issues)

1. **강결합**: Flappy Bird 로직이 엔진 코어에 하드코딩됨
2. **컴포넌트 혼재**: 범용 컴포넌트와 게임별 컴포넌트가 같은 파일에 존재
3. **GameState 특화**: Application이 Flappy Bird 전용 GameState에 의존
4. **재사용성 부족**: 새 프로젝트 시작 시 Flappy Bird 코드까지 포함됨

## 분리 전략 (Separation Strategy)

### Phase 1: Workspace 구조 분리
- Cargo workspace 설정
- 크레이트 단위로 프로젝트 분할

### Phase 2: 컴포넌트/시스템 분리
- 범용 컴포넌트와 게임별 컴포넌트 분리
- 범용 시스템과 게임별 시스템 분리

### Phase 3: Scene 시스템 도입
- GameState를 Scene 트레이트로 추상화
- Application의 범용화

## 새로운 프로젝트 구조 (New Project Structure)

```
flappy_nn/                     # 루트 프로젝트
├── Cargo.toml                 # Workspace 설정
├── README.md
├── REFACTOR_PLAN.md
├── assets/                    # 공용 에셋
│   ├── font/
│   ├── img/
│   └── shader/
├── engine/                    # 범용 게임 엔진 크레이트
│   ├── Cargo.toml
│   ├── src/
│   │   ├── lib.rs
│   │   ├── application.rs     # 범용 Application
│   │   ├── scene.rs           # Scene 트레이트 정의
│   │   ├── components/        # 범용 컴포넌트들
│   │   │   ├── mod.rs
│   │   │   ├── transform.rs
│   │   │   ├── collider.rs
│   │   │   ├── rendering.rs
│   │   │   └── physics.rs     # 2D Physics용 컴포넌트
│   │   ├── systems/           # 범용 시스템들
│   │   │   ├── mod.rs
│   │   │   ├── physics.rs
│   │   │   ├── collision.rs
│   │   │   └── rendering.rs
│   │   ├── resources/         # 범용 리소스들
│   │   │   ├── mod.rs
│   │   │   ├── camera.rs
│   │   │   ├── delta_time.rs
│   │   │   └── input_handler.rs
│   │   └── renderer/          # 렌더링 엔진 (기존과 동일)
│   │       ├── mod.rs
│   │       ├── renderer.rs
│   │       ├── mesh.rs
│   │       ├── texture.rs
│   │       └── ...
├── examples/
│   ├── flappy_bird/           # Flappy Bird 게임
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── main.rs
│   │   │   ├── flappy_scene.rs
│   │   │   ├── components/
│   │   │   │   ├── mod.rs
│   │   │   │   ├── player.rs
│   │   │   │   ├── pipe.rs
│   │   │   │   └── dna.rs
│   │   │   ├── systems/
│   │   │   │   ├── mod.rs
│   │   │   │   ├── process_nn.rs
│   │   │   │   ├── scroll_pipe.rs
│   │   │   │   └── ai_behavior.rs
│   │   │   └── resources/
│   │   │       ├── mod.rs
│   │   │       └── gene_handler.rs
│   │   └── assets/ -> ../../assets/  # 심볼릭 링크
│   └── physics_demo/          # 2D Physics 데모
│       ├── Cargo.toml
│       ├── src/
│       │   ├── main.rs
│       │   ├── physics_scene.rs
│       │   ├── components/
│       │   │   ├── mod.rs
│       │   │   ├── rigid_body.rs
│       │   │   └── physics_material.rs
│       │   └── systems/
│       │       ├── mod.rs
│       │       ├── physics_step.rs
│       │       └── collision_response.rs
│       └── assets/ -> ../../assets/  # 심볼릭 링크
```

## 핵심 인터페이스 설계 (Core Interface Design)

### Scene 트레이트
```rust
pub trait Scene {
    fn init(&mut self, world: &mut World);
    fn update(&mut self, world: &mut World, dt: f32);
    fn handle_input(&mut self, world: &mut World, input: &WindowEvent) -> bool;
    fn get_render_data(&self, world: &World) -> RenderData;
}
```

### 범용 Application
```rust
pub struct Application<T: Scene> {
    scene: T,
    renderer: RenderState,
    window: Arc<Window>,
    // ...
}
```

### Engine Builder 패턴
```rust
pub struct EngineBuilder {
    world: World,
}

impl EngineBuilder {
    pub fn register_component<T: Component>(&mut self) -> &mut Self;
    pub fn add_system<T: System + 'static>(&mut self, system: T) -> &mut Self;
    pub fn build<S: Scene>(self, scene: S) -> Engine<S>;
}
```

## 마이그레이션 단계 (Migration Steps)

### Step 1: Workspace 설정
- [ ] `Cargo.toml`에 workspace 설정 추가
- [ ] `engine/` 크레이트 생성
- [ ] `examples/flappy_bird/` 크레이트 생성

### Step 2: 렌더링 엔진 이동
- [ ] `src/renderer/` → `engine/src/renderer/` 이동
- [ ] 관련 의존성 업데이트

### Step 3: 범용 컴포넌트 분리
- [ ] `Transform`, `Collider`, `Tile` → `engine/src/components/`
- [ ] `Player`, `Pipe`, `DNA` → `examples/flappy_bird/src/components/`

### Step 4: 범용 시스템 분리
- [ ] 물리, 충돌, 렌더링 시스템 → `engine/src/systems/`
- [ ] Flappy Bird 전용 시스템 → `examples/flappy_bird/src/systems/`

### Step 5: Scene 시스템 도입
- [ ] `Scene` 트레이트 정의
- [ ] `FlappyScene` 구현
- [ ] `Application<T: Scene>` 범용화

### Step 6: 2D Physics 기반 구축
- [ ] Physics 컴포넌트 추가 (`RigidBody`, `Velocity`, `Mass`)
- [ ] Physics 시스템 구현
- [ ] `examples/physics_demo/` 생성

### Step 7: 테스트 및 검증
- [ ] Flappy Bird 기능 유지 확인
- [ ] Physics Demo 기본 동작 확인
- [ ] 빌드 시스템 검증

## 호환성 보장 (Compatibility Guarantees)

1. **기존 Flappy Bird 게임 100% 동작 보장**
2. **WASM 빌드 지원 유지**
3. **기존 에셋들 재사용 가능**
4. **성능 저하 없음**

## 다음 프로젝트 지원 (Future Project Support)

이 구조로 다음과 같은 프로젝트들을 쉽게 시작할 수 있습니다:

- **2D Physics Playground**: 강체 시뮬레이션, 충돌 처리
- **Platformer Game**: 점프 액션 게임
- **Particle System Demo**: 파티클 효과 시스템
- **UI Framework**: 게임 UI 컴포넌트 시스템

## 예상 작업 시간 (Estimated Timeline)

- **Step 1-2**: 1-2시간 (프로젝트 구조 설정)
- **Step 3-4**: 2-3시간 (컴포넌트/시스템 분리)
- **Step 5**: 2-3시간 (Scene 시스템 구현)
- **Step 6**: 1-2시간 (Physics 기반 구축)
- **Step 7**: 1시간 (테스트 및 검증)

**총 예상 시간**: 7-11시간

## 리스크 및 대응 방안 (Risks & Mitigation)

### 리스크
1. **복잡한 의존성 관계로 인한 빌드 실패**
2. **WASM 빌드 호환성 문제**
3. **성능 저하**

### 대응 방안
1. **점진적 마이그레이션**: 단계별로 진행하며 각 단계마다 빌드 확인
2. **feature flag 활용**: 조건부 컴파일로 호환성 유지
3. **벤치마크 테스트**: 리팩토링 전후 성능 비교

---

**작성일**: 2025-10-04  
**작성자**: Claude Code Assistant  
**목표 완료일**: TBD