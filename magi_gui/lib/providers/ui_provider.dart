import 'package:flutter_riverpod/flutter_riverpod.dart';

class UIState {
  final bool isSidebarVisible;
  final bool isContextPanelVisible;
  final String? selectedPdfPath;
  final String? selectedPdfTitle;

  UIState({
    this.isSidebarVisible = true,
    this.isContextPanelVisible = false,
    this.selectedPdfPath,
    this.selectedPdfTitle,
  });

  UIState copyWith({
    bool? isSidebarVisible,
    bool? isContextPanelVisible,
    String? selectedPdfPath,
    String? selectedPdfTitle,
  }) {
    return UIState(
      isSidebarVisible: isSidebarVisible ?? this.isSidebarVisible,
      isContextPanelVisible: isContextPanelVisible ?? this.isContextPanelVisible,
      selectedPdfPath: selectedPdfPath ?? this.selectedPdfPath,
      selectedPdfTitle: selectedPdfTitle ?? this.selectedPdfTitle,
    );
  }
}

class UINotifier extends StateNotifier<UIState> {
  UINotifier() : super(UIState());

  void toggleSidebar() {
    state = state.copyWith(isSidebarVisible: !state.isSidebarVisible);
  }

  void toggleContextPanel({String? path, String? title}) {
    if (path != null && title != null) {
      state = state.copyWith(
        isContextPanelVisible: true,
        selectedPdfPath: path,
        selectedPdfTitle: title,
      );
    } else {
      state = state.copyWith(isContextPanelVisible: !state.isContextPanelVisible);
    }
  }

  void closeContextPanel() {
    state = state.copyWith(isContextPanelVisible: false);
  }
}

final uiProvider = StateNotifierProvider<UINotifier, UIState>((ref) {
  return UINotifier();
});
